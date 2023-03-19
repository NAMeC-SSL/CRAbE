use crabe_io::communication::UsbTransceiver;
use crabe_protocol::protobuf::robot_packet::{IaToMainBoard, Kicker};
use std::thread;
use std::time::Duration;

fn main() {
    let mut usb =
        UsbTransceiver::new("/dev/ttyUSB0", 115_200).expect("Failed to create usb transceiver");

    loop {
        let packet = IaToMainBoard {
            robot_id: 0,
            normal_speed: 1.0,
            tangential_speed: 0.0,
            angular_speed: 1.0,
            motor_break: false,
            kicker_cmd: Kicker::Kick1.into(),
            kick_power: 0.3,
            charge: true,
            dribbler: true,
        };

        usb.send(packet);
        thread::sleep(Duration::from_millis(16));
    }
}
