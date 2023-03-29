use crabe_framework::data::world::game_state::{GameState, HaltedState};
use crabe_framework::data::world::game_state::GameState::Halted;
use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::World;
use crabe_protocol::protobuf::game_controller_packet::{GameEvent, Referee};
use crabe_protocol::protobuf::game_controller_packet::game_event::Event;
use crate::data::referee::RefereeCommand::Halt;

pub struct GameControllerFilter;

impl PostFilter for GameControllerFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {

        println!("bruh");

        let last_referee_packet = match filter_data.referee.last() {
            None => {
                return;
            }
            Some(r) => r
        };

        let last_game_event =  match last_referee_packet.game_events.last() {
                None => {
                    return;
                }
                Some(e) => e
            };

        // if let Halt = last_event {
        //
        // }

        let last_command = &last_referee_packet.command();
        let last_event = last_game_event.event.clone().unwrap();

        dbg!(last_command);
        dbg!(last_event);

        // match &world.data.state {
        //     GameState::Halted(h) => {
        //         match h {
        //             HaltedState::Halt => {
        //                 match last_event {
        //
        //                 }
        //             }
        //             HaltedState::Timeout => {}
        //         }
        //     }
        //     GameState::Stopped(s) => {
        //
        //     }
        //     GameState::Running(r) => {
        //
        //     }
        // }
    }
}
