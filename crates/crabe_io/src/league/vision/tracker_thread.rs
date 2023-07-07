use crate::communication::MulticastUDPReceiver;
use crate::constant::{TRACKED_PORT, VISION_PORT_REAL, VISION_PORT_SIM};
use crate::league::vision::VisionConfig;
use crate::pipeline::input::ReceiverTask;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::input::InboundData;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;
use log::{error, info};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;
use crabe_protocol::protobuf::tracker_vision_packet::TrackerWrapperPacket;

// TODO: Document
pub struct TrackerVision {
    rx_tracker: Receiver<TrackerWrapperPacket>,
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl TrackerVision {
    pub fn with_config(vision_cfg: &VisionConfig, common_cfg: &CommonConfig) -> Self {
        let port = if let Some(port) = vision_cfg.tracker_port {
            port
        } else {
            TRACKED_PORT
        };

        let (tx_tracker, rx_tracker) = mpsc::channel::<TrackerWrapperPacket>();
        let ipv4 = Ipv4Addr::from_str(vision_cfg.tracker_ip.as_str())
            .expect("Failed to create an ipv4 address with the ip");
        let mut tracker =
            MulticastUDPReceiver::new(ipv4, port).expect("Failed to create vision receiver");

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);

        let handle = thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                if let Some(packet) = tracker.receive() {
                    if let Err(e) = tx_tracker.send(packet) {
                        error!("Error sending Tracker packet: {:?}", e);
                    }
                }
            }
        });

        Self {
            rx_tracker,
            handle: Some(handle),
            running,
        }
    }
}

impl ReceiverTask for TrackerVision {
    fn fetch(&mut self, input: &mut InboundData) {
        input.tracker_packet.extend(self.rx_tracker.try_iter());
    }

    fn close(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            match handle.join() {
                Ok(_) => info!("Tracker Thread finished successfully"),
                Err(e) => println!("Tracker thread finished with an error: {:?}", e),
            }
        }
    }
}
