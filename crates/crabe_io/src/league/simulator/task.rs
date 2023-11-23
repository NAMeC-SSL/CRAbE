use crate::communication::UDPTransceiver;

use crate::league::simulator::config::SimulatorConfig;

use crabe_framework::data::output::{Command, CommandMap, Feedback, FeedbackMap, Kick};

use crabe_protocol::protobuf::simulation_packet::{
    robot_move_command, MoveLocalVelocity, RobotCommand, RobotControl, RobotControlResponse,
    RobotMoveCommand,
};
use log::debug;

use std::net::Ipv4Addr;

use crate::constant::{SIM_PORT_BLUE, SIM_PORT_YELLOW};
use crabe_framework::config::CommonConfig;
use crabe_framework::constant::MAX_ID_ROBOTS;

use crate::pipeline::output::CommandSenderTask;

pub struct Simulator {
    socket: UDPTransceiver,
}

impl Simulator {
    pub fn with_config(simulator_cfg: SimulatorConfig, common_cfg: &CommonConfig) -> Self {
        let port;
        if let Some(sim_port) = simulator_cfg.simulator_port {
            port = sim_port
        } else if common_cfg.yellow {
            port = SIM_PORT_YELLOW;
        } else {
            port = SIM_PORT_BLUE;
        }

        let socket =
            UDPTransceiver::new(Ipv4Addr::LOCALHOST, port).expect("Failed to setup simulator");

        Self { socket }
    }

    fn prepare_packet(&self, commands: impl Iterator<Item = (u8, Command)>) -> RobotControl {
        let mut packet = RobotControl::default();

        for (id, command) in commands {
            let (kick_speed, kick_angle) = match &command.kick {
                None => (0.0, 0.0),
                Some(Kick::StraightKick { power }) => (*power, 0.0),
                Some(Kick::ChipKick { power }) => (*power, 45.0),
            };

            let robot_command = RobotCommand {
                id: id as u32,
                move_command: Some(RobotMoveCommand {
                    command: Some(robot_move_command::Command::LocalVelocity(
                        MoveLocalVelocity {
                            forward: command.forward_velocity,
                            left: command.left_velocity,
                            angular: command.angular_velocity,
                        },
                    )),
                }),
                kick_speed: Some(kick_speed),
                kick_angle: Some(kick_angle),
                dribbler_speed: Some(command.dribbler),
            };
            packet.robot_commands.push(robot_command);
        }

        packet
    }

    fn fetch(&mut self) -> FeedbackMap {
        let mut feedback_map: FeedbackMap = Default::default();
        if let Some(packet) = self.socket.receive::<RobotControlResponse>() {
            for robot_feedback in packet.feedback {
                debug!(
                    "assigned feedback {:?} to robot #{}",
                    robot_feedback, robot_feedback.id
                );

                feedback_map.insert(
                    robot_feedback.id,
                    Feedback {
                        has_ball: robot_feedback.dribbler_ball_contact(),
                        voltage: Default::default(),
                    },
                );
            }
        }

        feedback_map
    }
}

impl CommandSenderTask for Simulator {
    fn step(&mut self, commands: CommandMap) -> FeedbackMap {
        let packet = self.prepare_packet(commands.into_iter());
        self.socket.send(packet);
        self.fetch()
    }

    fn close(&mut self) {
        let mut commands: CommandMap = Default::default();
        for id in 0..MAX_ID_ROBOTS {
            commands.insert(id as u8, Default::default());
        }

        self.step(commands);
    }
}
