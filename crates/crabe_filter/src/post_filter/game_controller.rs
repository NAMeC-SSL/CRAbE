use std::time::Instant;
use chrono::Duration;
use crabe_framework::data::world::game_state::{GameState, HaltedState, RunningState, StoppedState};
use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::{TeamColor, World};
use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::game_controller_packet::referee::Command;
use crate::data::referee::RefereeCommand::Stop;

#[derive(Default)]
pub struct GameControllerPostFilter {
    chrono: Option<Instant>
}

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

        if let Some(blue_team_on_positive_half) = last_referee_packet.blue_team_on_positive_half {
            if blue_team_on_positive_half {
               world.data.positive_half = TeamColor::Blue
            } else {
                world.data.positive_half = TeamColor::Yellow
            }
        };

        // from any state
        if let Command::Halt = referee_command {
            world.data.state = GameState::Halted(HaltedState::Halt);
        }

        match referee_command {
            Command::Halt => {
                world.data.state = GameState::Halted(HaltedState::Halt);
            }
            Command::Stop => {
                world.data.state = GameState::Stopped(StoppedState::Stop);
            }
            _ => {}
        }

        let mut kicker_team = None;

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
                            Command::PrepareKickoffBlue => {
                                kicker_team = Some(TeamColor::Blue);
                                if world.team_color == TeamColor::Blue {
                                    world.data.state = GameState::Stopped(StoppedState::PrepareKickoff);
                                } else {
                                    world.data.state = GameState::Stopped(StoppedState::Stop);
                                }
                            }
                            Command::PrepareKickoffYellow => {
                                kicker_team = Some(TeamColor::Yellow);
                                if world.team_color == TeamColor::Yellow {
                                    world.data.state = GameState::Stopped(StoppedState::PrepareKickoff);
                                } else {
                                    world.data.state = GameState::Stopped(StoppedState::Stop);
                                }
                            }
                            Command::PreparePenaltyBlue => {}
                            Command::PreparePenaltyYellow => {}
                            Command::NormalStart => {
                                world.data.state = GameState::Running(RunningState::KickOff(kicker_team.unwrap_or(TeamColor::Blue))); // FIX THIS auto color under too
                            }
                            _ => {}
                        }
                    }
                    StoppedState::PrepareKickoff => {
                        if let Command::NormalStart = referee_command {
                            world.data.state = GameState::Running(RunningState::KickOff(kicker_team.unwrap_or(TeamColor::Blue)));
                        }
                    }
                    StoppedState::PreparePenalty => {
                        if let Command::NormalStart = referee_command {
                            world.data.state = GameState::Running(RunningState::KickOff(kicker_team.unwrap_or(TeamColor::Blue)));
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
                    RunningState::KickOff(_) => {
                        if let Some(ball) = &world.ball {
                            if ball.velocity.xy().norm() > 0.3 {
                                world.data.state = GameState::Running(RunningState::Run);
                            }
                        }
                        if let Some(chrono) = &self.chrono {
                            if chrono.elapsed() > std::time::Duration::from_secs(10) {
                                world.data.state = GameState::Running(RunningState::Run);
                            }
                        } else {
                            // start chrono
                            self.chrono = Some(Instant::now());
                        }
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
        // world.data.state = GameState::Running(RunningState::Run);
    }
}
