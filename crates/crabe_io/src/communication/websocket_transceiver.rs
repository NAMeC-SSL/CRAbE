use flume::{unbounded, Receiver, Sender};
use futures_util::StreamExt;
use log::{error, info};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::net::SocketAddr;
use std::thread;
use std::thread::JoinHandle;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio::task::JoinSet;
use tungstenite::Message;

pub struct WebSocketThread<RX, TX> {
    addr: SocketAddr,
    rx: Receiver<TX>,
    tx: Sender<RX>,
    runtime: Runtime,
}

impl<RX: Send + DeserializeOwned, TX: Send + Serialize> WebSocketThread<RX, TX> {
    fn new(addr: SocketAddr, rx: Receiver<TX>, tx: Sender<RX>) -> Self {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        Self {
            addr,
            rx,
            tx,
            runtime,
        }
    }

    async fn handle_connection(
        raw_stream: TcpStream,
        addr: SocketAddr,
        rx: Receiver<TX>,
        tx: Sender<RX>,
    ) -> Result<(), tungstenite::Error> {
        info!("Incoming TCP connection from: {}", addr);
        let ws_stream = tokio_tungstenite::accept_async(raw_stream).await?;
        info!("WebSocket connection established: {}", addr);

        let (outgoing, incoming) = ws_stream.split();
        let incoming_fut = incoming
            .filter_map(|x| async move {
                if let Ok(Message::Text(m)) = x {
                    if let Ok(req) = serde_json::from_str::<RX>(&m) {
                        return Some(Ok(req));
                    }
                }

                None
            })
            .forward(tx.sink());
        let outgoing_fut = rx
            .stream()
            .map(|m| Ok(Message::text(serde_json::to_string(&m).unwrap())))
            .forward(outgoing);
        let (_incoming_res, _outgoing_res) = tokio::join!(incoming_fut, outgoing_fut);

        info!("{} disconnected", &addr);

        Ok(())
    }
}

impl<RX: Send + DeserializeOwned + 'static, TX: Send + Serialize + 'static>
    WebSocketThread<RX, TX>
{
    fn run(&mut self, cancellation: Receiver<()>) {
        let future = async {
            let try_socket = TcpListener::bind(&self.addr).await;
            let listener = try_socket.expect("Failed to bind");
            info!("Listening on: {}", self.addr);
            let receive = async {
                let mut join_set = JoinSet::new();
                while let Ok((stream, addr)) = listener.accept().await {
                    join_set.spawn(Self::handle_connection(
                        stream,
                        addr,
                        self.rx.clone(),
                        self.tx.clone(),
                    ));
                }

                while let Some(res) = join_set.join_next().await {
                    if let Err(join_error) = res {
                        error!("Join error: #{join_error}")
                    } else if let Ok(Err(connection_error)) = res {
                        error!("Connection error: #{connection_error}")
                    }
                }
            };

            tokio::select! {
                _ = receive => {}
                _ = cancellation.recv_async() => {}
            };
        };
        self.runtime.block_on(future);
    }
}

pub struct WebSocketTransceiver<RX, TX> {
    rx: Receiver<RX>,
    tx: Sender<TX>,
    handle: JoinHandle<()>,
    cancellation_tx: Sender<()>,
}

impl<RX: DeserializeOwned + Send + 'static, TX: Serialize + Send + 'static>
    WebSocketTransceiver<RX, TX>
{
    pub fn spawn(addr: SocketAddr) -> Self {
        let (task_tx, transceiver_rx) = unbounded();
        let (transceiver_tx, task_rx) = unbounded();
        let (cancellation_tx, cancellation_rx) = unbounded();

        let mut websocket_thread = WebSocketThread::new(addr, task_rx, task_tx);
        let handle = thread::spawn(move || {
            websocket_thread.run(cancellation_rx);
        });

        Self {
            rx: transceiver_rx,
            tx: transceiver_tx,
            cancellation_tx,
            handle,
        }
    }

    pub fn send(&mut self, msg: TX) {
        self.tx
            .send(msg)
            .unwrap_or_else(|x| error!("Send error: #{x}"));
    }

    pub fn receive(&mut self) -> Option<RX> {
        self.rx.try_recv().ok()
    }

    pub fn close(self) {
        self.cancellation_tx
            .send(())
            .unwrap_or_else(|_e| error!("Error sending cancellation"));
        self.handle.join().unwrap_or_else(|_e| error!("Join error"));
    }
}
