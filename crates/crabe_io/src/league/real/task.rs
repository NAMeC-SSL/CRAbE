use crate::league::real::RealConfig;

use crabe_framework::constant::MAX_ID_ROBOTS;
use crabe_framework::data::output::{Command, CommandMap, FeedbackMap, Kick};

use crabe_protocol::protobuf::robot_packet::IaToMainBoard;

use crate::communication::UsbTransceiver;
use crate::pipeline::output::CommandSenderTask;

pub struct Real {
    usb: UsbTransceiver,
}

impl Real {
    pub fn with_config(usb_config: RealConfig) -> Self {
        let usb = UsbTransceiver::new(&usb_config.usb_port, usb_config.usb_baud)
            .expect("Failed to create usb transceiver");

        Self { usb }
    }

    fn prepare_packet(&mut self, id: u8, command: Command) -> IaToMainBoard {
        let (kicker_cmd, kick_power) = match command.kick {
            None => {
                (0, 0.0 as f32) // TODO : Remove this 0 and use the kicker enum
            }
            Some(Kick::StraightKick { power }) => (2, power),
            Some(Kick::ChipKick { power }) => (1, power),
        };

        dbg!(IaToMainBoard {
            robot_id: id as u32,
            normal_speed: command.forward_velocity,
            tangential_speed: command.left_velocity,
            angular_speed: command.angular_velocity,
            motor_break: false,
            kicker_cmd,
            kick_power,
            charge: true, //command.charge,
            dribbler: command.dribbler > 0.0,
        })
    }
}

impl CommandSenderTask for Real {
    fn step(&mut self, commands: CommandMap) -> FeedbackMap {
        for (id, command) in commands.into_iter() {
            let packet = self.prepare_packet(id, command);
            self.usb.send(packet);
        }
        Default::default()
    }

    fn close(&mut self) {
        let mut commands: CommandMap = Default::default();
        for id in 0..MAX_ID_ROBOTS {
            commands.insert(id as u8, Default::default());
        }

        self.step(commands);
    }
}
