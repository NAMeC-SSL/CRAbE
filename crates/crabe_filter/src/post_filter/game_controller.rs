use std::time::Instant;
use log::warn;
use crabe_framework::data::world::{GameState, HaltedState, TeamColor, World};
use crabe_framework::data::world::game_state::{RunningState, StoppedState};
use crate::constant::{KICK_TIMEOUT, PENALTY_TIMEOUT};
use crate::data::FilterData;
use crate::data::referee::event::{Event, GameEvent};
use crate::data::referee::RefereeCommand;
use crate::post_filter::PostFilter;

#[derive(Default)]
pub struct GameControllerPostFilter {

}

impl GameControllerPostFilter {
    fn halted(state: HaltedState, command: Option<&RefereeCommand>, event: Option<&Event>) -> GameState {
        match command {
            Some(RefereeCommand::Stop) => {
                GameState::Stopped(StoppedState::Stop)
            }

            Some(_) => {
                warn!("invalid command {:?}", command);
                GameState::Halted(state)
            }

            None => { GameState::Halted(state) }
        }
    }

    fn stopped(state: StoppedState, command: Option<&RefereeCommand>, _event: Option<&Event>) -> GameState {
        match state {
            StoppedState::Stop => {
                match command {
                    Some(RefereeCommand::Timeout(_team_color)) => {
                        GameState::Halted(HaltedState::Timeout)
                    }

                    Some(RefereeCommand::BallPlacement(team_color)) => {
                        GameState::Stopped(StoppedState::BallPlacement(
                            *team_color,
                            Instant::now(),
                        ))
                    }

                    Some(RefereeCommand::PrepareKickoff(team_color)) => {
                        GameState::Stopped(StoppedState::PrepareKickoff(*team_color))
                    }

                    Some(RefereeCommand::PreparePenalty(team_color)) => {
                        GameState::Stopped(StoppedState::PreparePenalty(*team_color))
                    }

                    Some(_) => {
                        warn!("invalid command {:?}", command);
                        GameState::Stopped(state)
                    }
                    None => {
                        GameState::Stopped(state)
                    }
                }
            }

            StoppedState::BallPlacement(team_color, start) => {
                match command {
                    Some(RefereeCommand::Stop) => {
                        GameState::Stopped(StoppedState::Stop)
                    }

                    Some(RefereeCommand::DirectFree(team_color)) => {
                        GameState::Running(RunningState::FreeKick(
                            *team_color,
                            Instant::now()
                        ))
                    }

                    // TODO: Other commands?

                    Some(_) => {
                        warn!("invalid command {:?}", command);
                        GameState::Stopped(state)
                    }

                    None => { GameState::Stopped(state) }
                }
            }

            StoppedState::PrepareKickoff(team_color) => {
                match command {
                    Some(RefereeCommand::NormalStart) => {
                        GameState::Running(RunningState::KickOff(
                            team_color,
                            Instant::now(),
                        ))
                    }

                    Some(_) => {
                        warn!("invalid command {:?}", command);
                        GameState::Stopped(state)
                    }
                    None => { GameState::Stopped(state) }
                }
            }

            StoppedState::PreparePenalty(team_color) => {
                match command {
                    Some(RefereeCommand::NormalStart) => {
                        GameState::Running(RunningState::Penalty(
                            team_color,
                            Instant::now(),
                        ))
                    }

                    Some(_) => {
                        warn!("invalid command");
                        GameState::Stopped(state)
                    }
                    None => { GameState::Stopped(state) }
                }
            }
        }
    }

    fn running(state: RunningState, command: Option<&RefereeCommand>, event: Option<&Event>, team_color: TeamColor) -> GameState {
        match state {
            RunningState::KickOff(_team_color, start)
            | RunningState::FreeKick(_team_color, start) => {
                if start.elapsed() >= KICK_TIMEOUT {
                    GameState::Running(RunningState::Run)
                } else {
                    GameState::Running(state)
                }

                // TODO: Handle moved ball

            }

            RunningState::Penalty(_team_color, start) => {
                if start.elapsed() >= PENALTY_TIMEOUT {
                    GameState::Running(RunningState::Run)
                } else {
                    GameState::Running(state)
                }
            }

            RunningState::Run => {
                match event {
                    Some(Event::Goal(goal_info)) => {
                        GameState::Stopped(StoppedState::PrepareKickoff(goal_info.by_team.opposite()))
                    }

                    Some(Event::BallLeftFieldTouchLine(left_field_info))
                    | Some(Event::BallLeftFieldGoalLine(left_field_info)) => {
                        GameState::Stopped(StoppedState::BallPlacement(
                            left_field_info.by_team.opposite(),
                            Instant::now(),
                        ))
                    }

                    Some(Event::BotTippedOver(bot_tipped_over_info)) => {
                        // Precaution : stopping robots if there was an accident
                        // Robot has to be substituted
                        if bot_tipped_over_info.by_team.eq(&team_color) {
                            warn!("[IRL ACCIDENT] Robot {}", bot_tipped_over_info.by_bot.unwrap());
                        }
                        GameState::Halted(HaltedState::Halt)
                    }

                    _ => {
                        match command {
                            Some(RefereeCommand::Stop) => {
                                GameState::Stopped(StoppedState::Stop)
                            }

                            Some(_) => {
                                warn!("invalid command {:?}", command);
                                GameState::Running(state)
                            }

                            None => { GameState::Running(state) }
                        }
                    }
                }
            }
        }
    }

    fn handle(state: GameState, command: Option<&RefereeCommand>, event: Option<&Event>, team_color: TeamColor) -> GameState {
        match command {
            Some(RefereeCommand::Halt) => GameState::Halted(HaltedState::Halt),
            _ => {
                match state {
                    GameState::Running(running_state) => Self::running(running_state, command, event, team_color),
                    GameState::Halted(halted_state) => Self::halted(halted_state, command, event),
                    GameState::Stopped(stopped_state) => Self::stopped(stopped_state, command, event)
                }
            }
        }
    }
}

impl PostFilter for GameControllerPostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let last_referee_packet = filter_data.referee.last();
        let event = last_referee_packet.map(|r| r.game_events.last().map(|e| &e.event)).flatten();
        let ref_command = last_referee_packet.map(|r| &r.command);
        // TODO: should we put this somewhere else?
        if let Some(team_on_positive_half) = last_referee_packet.map(|r| r.positive_half).flatten() {
            world.data.positive_half = team_on_positive_half
        }
        world.data.state = Self::handle(world.data.state, ref_command, event, world.team_color)
        // TODO : ally and enemy
    }
}