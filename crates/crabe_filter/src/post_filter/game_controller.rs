use crabe_framework::data::world::game_state::{GameState, HaltedState, RunningState, StoppedState};
use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::World;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::game_controller_packet::referee::Command;

pub struct GameControllerPostFilter;

impl PostFilter for GameControllerPostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {

        let last_referee_packet = match filter_data.referee.last() {
            None => {
                return;
            }
            Some(r) => r
        };

        let last_game_event =  last_referee_packet.game_events.last();

        let referee_command = last_referee_packet.command();

        // from any state
        if let Command::Halt = referee_command {
            world.data.state = GameState::Running(RunningState::Run);
        }

        // match referee_command {
        //     Command::Halt => {
        //         world.data.state = GameState::Halted(HaltedState::Halt);
        //     }
        //     Command::Stop => {
        //         world.data.state = GameState::Stopped(StoppedState::Stop);
        //     }
        //     _ => {}
        // }

        match &world.data.state {
            GameState::Halted(_) => {
            }
            GameState::Stopped(stopped_state) => {
                match stopped_state {
                    StoppedState::Stop => {
                        // TODO: prepare kickoffs :(
                        match referee_command {
                            Command::ForceStart => {
                                world.data.state = GameState::Running(RunningState::Run);
                            }
                            Command::PrepareKickoffBlue => {}
                            Command::PrepareKickoffYellow => {}
                            Command::PreparePenaltyBlue => {}
                            Command::PreparePenaltyYellow => {}
                            _ => {}
                        }
                    }
                    StoppedState::PrepareKickoff => {
                        if let Command::NormalStart = referee_command {
                            world.data.state = GameState::Running(RunningState::KickOff);
                        }
                    }
                    StoppedState::PreparePenalty => {
                        if let Command::NormalStart = referee_command {
                            world.data.state = GameState::Running(RunningState::KickOff);
                        }
                    }
                    StoppedState::BallPlacement => {
                        // TODO: what the fuck ?
                        // if let Command::Continue {
                        //     world.data.state = GameState::Running(RunningState::KickOff);
                        // }
                    }
                }
            }
            GameState::Running(running_state) => {
                match running_state {
                    RunningState::KickOff => {
                        // if let Command::AfterXSecondsOrIfBallMoved {
                        //     world.data.state = GameState::Running(RunningState::Run);
                        // }
                    }
                    RunningState::FreeKick => {
                        // if let Command::AfterXSecondsOrIfBallMoved {
                        //     world.data.state = GameState::Running(RunningState::Run);
                        // }
                    }
                    _ => {}
                }
            }
        }

        dbg!(referee_command);
        dbg!(&world.data.state);
        world.data.state = GameState::Running(RunningState::Run);
    }
}
