use crabe_io::communication::UDPTransceiver;
use crabe_protocol::protobuf::simulation_packet::robot_move_command::Command::LocalVelocity;
use crabe_protocol::protobuf::simulation_packet::{
    MoveLocalVelocity, RobotCommand, RobotControl, RobotControlResponse, RobotMoveCommand,
};
use std::net::Ipv4Addr;

fn create_commands() -> RobotControl {
    let robot_command = RobotCommand {
        id: 0,
        move_command: Some(RobotMoveCommand {
            command: Some(LocalVelocity(MoveLocalVelocity {
                forward: 0.0,
                left: 0.0,
                angular: 1.0,
            })),
        }),
        kick_speed: None,
        kick_angle: None,
        dribbler_speed: None,
    };

    RobotControl {
        robot_commands: vec![robot_command],
    }
}

fn main() {
    let mut udp_transceiver = UDPTransceiver::new(Ipv4Addr::LOCALHOST, 10301)
        .expect("Cannot create a socket UDP Receiver");
    loop {
        udp_transceiver.send::<RobotControl>(create_commands());
        // dbg!(udp_transceiver.receive::<RobotControlResponse>());
    }
}
