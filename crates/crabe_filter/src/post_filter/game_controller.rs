use crabe_framework::data::world::game_state::{GameState, HaltedState};
use crabe_framework::data::world::game_state::GameState::Halted;
use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::World;
use crabe_protocol::protobuf::game_controller_packet::{GameEvent, Referee};
use crate::data::referee::RefereeCommand::Halt;

pub struct GcFilter;

impl PostFilter for GcFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {


        let last_event = match filter_data.referee.last() {
            None => {
                return;
            }
            Some(e) => match e.game_events.last() {
                None => {
                    return;
                }
                Some(e) => e
            }
        };

        if let Halt = last_event {

        }

        match &world.data.state {
            GameState::Halted(h) => {
                match h {
                    HaltedState::Halt => {

                    }
                    HaltedState::Timeout => {}
                }
            }
            GameState::Stopped(s) => {

            }
            GameState::Running(r) => {

            }
        }
    }
}
