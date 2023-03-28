use crate::communication::MulticastUDPReceiver;
use crate::league::game_controller::GameControllerConfig;
use crate::pipeline::input::ReceiverTask;
use crabe_framework::data::input::InboundData;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use log::{error, info};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;

// TODO: Document
pub struct GameController {
    rx_gc: Receiver<Referee>,
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl GameController {
    pub fn with_config(cli: GameControllerConfig) -> Self {
        let (tx_gc, rx_gc) = mpsc::channel::<Referee>();
        let ipv4 = Ipv4Addr::from_str(cli.gc_ip.as_str())
            .expect("Failed to create an ipv4 address with the ip");
        let mut gc =
            MulticastUDPReceiver::new(ipv4, cli.gc_port).expect("Failed to create gc receiver");
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);

        let handle = thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                if let Some(packet) = gc.receive() {
                    if let Err(e) = tx_gc.send(packet) {
                        error!("Error sending GameController packet: {:?}", e);
                    }
                }
            }
        });

        Self {
            rx_gc,
            handle: Some(handle),
            running,
        }
    }
}

impl ReceiverTask for GameController {
    fn fetch(&mut self, input: &mut InboundData) {
        input.gc_packet.extend(self.rx_gc.try_iter());
    }

    fn close(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            match handle.join() {
                Ok(_) => info!("GC Thread finished successfully"),
                Err(e) => println!("GC thread finished with an error: {:?}", e),
            }
        }
    }
}
