use crate::league::real::RealConfig;
use log::error;

use crabe_framework::constant::MAX_ID_ROBOTS;
use crabe_framework::data::output::{Command, CommandMap, FeedbackMap, Kick};

use crabe_protocol::protobuf::robot_packet::{BaseCommand, Kicker, PcToBase};

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

    fn prepare_packet(&mut self, commands: impl Iterator<Item = (u8, Command)>) -> PcToBase {
        let mut packet = PcToBase::default();
        for (id, command) in commands {
            let (kicker_cmd, kick_power) = match command.kick {
                None => (Kicker::NoKick, 0.0_f32),
                Some(Kick::StraightKick { power }) => (Kicker::Flat, power),
                Some(Kick::ChipKick { power }) => (Kicker::Chip, power),
            };

            packet.commands.push(BaseCommand {
                robot_id: id as u32,
                normal_velocity: command.forward_velocity,
                tangential_velocity: command.left_velocity,
                angular_velocity: command.angular_velocity,
                kick: kicker_cmd.into(),
                kick_power,
                charge: command.charge,
                dribbler: command.dribbler,
            });
        }
        packet
    }
}

impl CommandSenderTask for Real {
    fn step(&mut self, commands: CommandMap) -> FeedbackMap {
        dbg!(&commands);
        if commands.len() > 16 {
            error!("Capacity oversize for the commands !");
            Default::default()
        }

        let packet = self.prepare_packet(commands.into_iter());
        self.usb.send(packet);

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
