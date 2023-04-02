use std::time::Instant;
use std::u8;
use chrono::Duration;
use crabe_framework::data::world::game_state::{GameState, HaltedState, RunningState, StoppedState};
use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::{TeamColor, World};
use crabe_protocol::protobuf::game_controller_packet::game_event::Event;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::game_controller_packet::referee::Command;
use crate::data::event::GameEvent;
use crate::data::referee::RefereeCommand;
use crate::data::referee::RefereeCommand::Stop;

#[derive(Debug)]
pub struct GameControllerPostFilter {
    previous_game_event: crabe_protocol::protobuf::game_controller_packet::GameEvent,
    previous_event: Option<Event>,
    chrono: Option<Instant>,
    kicked_off_once: bool
}

impl Default for GameControllerPostFilter {
    fn default() -> Self {
        GameControllerPostFilter {
            previous_game_event: Default::default(),
            previous_event: None,
            chrono: Option::from(Instant::now()),
            kicked_off_once: false,
        }
    }
}

/// Contains all the possible actions that we may be able
/// to execute from a referee command
/// The job of these functions is just to change
/// the current world state according to the command
/// and the last event that occurred
impl GameControllerPostFilter {
    fn halt_state_branch(world: &mut World) {
        // Halt event, all robots should stop
        // Done: 2/2
        // consider that timeout is equivalent to normal halt
        world.data.state = GameState::Halted(HaltedState::Halt);
        println!("Stop all robots");
    }

    fn stop_state_branch(previous_event_opt: &Option<Event>, world: &mut World, mut kicked_off_once: bool) {
        // TODO: 3/4 ?
        if let Some(previous_event) = previous_event_opt {
            match previous_event {
                // Goal has been scored, prepare for next kickoff phase
                Event::Goal { .. } => {
                    world.data.state = GameState::Stopped(StoppedState::PrepareKickoff);
                    println!("Prepare for kickoff");
                }

                Event::BallLeftFieldTouchLine { .. } => {
                    world.data.state = GameState::Stopped(StoppedState::BallPlacement);
                    println!("Ball got out of the field by the touch lines !");
                }

                Event::BallLeftFieldGoalLine { .. } => {
                    world.data.state = GameState::Stopped(StoppedState::BallPlacement);
                    println!("Ball got out of the field by the goal lines !");
                }

                &_ => {}
            }
        } else {
            // Particularly, it should be None when we just started the match
            // Thus, it's a kickoff
            if !kicked_off_once {
                world.data.state = GameState::Stopped(StoppedState::PrepareKickoff);
                kicked_off_once = true;
            } else {
                // this one's totally arbitrary
                // i don't understand how we can fetch a forced free kick from the commands
                // todo: fix what's mentioned above me (fix me !)
                world.data.state = GameState::Running(RunningState::FreeKick);
            }
        }
    }

    fn normal_start_state_branch(previous_event_opt: &Option<Event>, world: &mut World, mut chrono: Option<Instant>) {
        if let Some(previous_event) = previous_event_opt {
            match previous_event {
                Event::Goal(g) => {
                    dbg!(g.by_team);
                    // Kickoff is in progress by a team, place accordingly on your side
                    // 10s until we go into normal state
                    if let Some(chrono) = chrono {
                        println!("Kickoff in progress ! It lasts for 10s at most");
                        if chrono.elapsed() > std::time::Duration::from_secs(10) {
                            // let kickoff_team = g.by_team as TeamColor;
                            // world.data.state = GameState::Running(RunningState::KickOff(kickoff_team));
                            world.data.state = GameState::Running(RunningState::KickOff(TeamColor::Blue));
                        }
                    } else {
                        println!("Running normally after kickoff");
                        // start chrono
                        chrono = Some(Instant::now());
                        world.data.state = GameState::Running(RunningState::Run);
                    }
                }

                &_ => {
                    // Just play the game when no particular state is found
                    // - what's your problem green ?
                    // - me said alone ramp, me said alone ramp
                    // - (proceeds to destroy his table)
                    world.data.state = GameState::Running(RunningState::Run);
                }
            }
        } else {
            println!("Play by default");
            world.data.state = GameState::Running(RunningState::Run);
        }
    }

    fn timeout_yellow_branch(world: &mut World) {
        world.data.state = GameState::Halted(HaltedState::Timeout);
    }
    fn timeout_blue_branch(world: &mut World) {
        world.data.state = GameState::Halted(HaltedState::Timeout);
    }

    fn freekick_blue_branch(world: &mut World, mut chrono_opt: Option<Instant>) {
        if let Some(chrono) = chrono_opt {
            // if 10s have passed, game runs normally
            if chrono.elapsed() > std::time::Duration::from_secs(10) {
                world.data.state = GameState::Running(RunningState::Run);
                chrono_opt = Some(Instant::now());
            } else {
                // otherwise, we are still in the FreeKick state
                world.data.state = GameState::Running(RunningState::FreeKick);
            }
        }
        unreachable!("Chrono is None!");
    }

    fn freekick_yellow_branch(world: &mut World, mut chrono_opt: Option<Instant>) {
        if let Some(chrono) = chrono_opt {
            if chrono.elapsed() > std::time::Duration::from_secs(10) {
                world.data.state = GameState::Running(RunningState::Run);
                chrono_opt = Some(Instant::now());
            } else {
                world.data.state = GameState::Running(RunningState::FreeKick);
            }
        }
    }

    fn ball_placement_blue_branch(world: &mut World, chrono_opt: Option<Instant>) {
        if let Some(chrono) = chrono_opt {
            // [ALLEMAGNE] chrono check peut être enlevé si pas de ball placement auto
            if chrono.elapsed() >= Duration::seconds(30) {
                world.data.state = GameState::Running(RunningState::Run);
            } else {
                world.data.state = GameState::Stopped(StoppedState::BallPlacement);
            }
        }
        // The chrono should always be available to us
        unreachable!();
    }
}



impl PostFilter for GameControllerPostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {

        // grab data
        let last_referee_packet = match filter_data.referee.last() {
            None => {
                return;
            }
            Some(r) => r
        };

        // Note de journal: 4h44
        // - touche une pute :D
        // dbg!(last_referee_packet);
        let ref_command = last_referee_packet.command();

        // dbg!(&ref_command);
        dbg!(&self.previous_event);

        match ref_command {
            Command::Halt => GameControllerPostFilter::halt_state_branch(world),
            Command::Stop => GameControllerPostFilter::stop_state_branch(&self.previous_event, world, self.kicked_off_once),
            Command::NormalStart => GameControllerPostFilter::normal_start_state_branch(&self.previous_event, world, self.chrono),
            Command::TimeoutBlue => GameControllerPostFilter::timeout_blue_branch(world),
            Command::TimeoutYellow => GameControllerPostFilter::timeout_yellow_branch(world),
            Command::DirectFreeBlue => GameControllerPostFilter::freekick_blue_branch(world, self.chrono),
            Command::DirectFreeYellow => GameControllerPostFilter::freekick_yellow_branch(world, self.chrono),
            Command::BallPlacementBlue => GameControllerPostFilter::ball_placement_blue_branch(world, self.chrono),
            _ => {}
        }

        // Update previous gamestate & event
        if let Some(previous_game_event) = last_referee_packet.game_events.last() {
            self.previous_game_event = previous_game_event.clone();
            self.previous_event = previous_game_event.event.clone(); //todo: don't clone this, specify lifetime
        }

        let referee_command = last_referee_packet.command();

        // if let Some(blue_team_on_positive_half) = last_referee_packet.blue_team_on_positive_half {
        //     if blue_team_on_positive_half {
        //        world.data.positive_half = TeamColor::Blue
        //     } else {
        //         world.data.positive_half = TeamColor::Yellow
        //     }
        // };
        //
        // // from any state
        // if let Command::Halt = referee_command {
        //     world.data.state = GameState::Halted(HaltedState::Halt);
        // }
        //
        // match referee_command {
        //     Command::Halt => {
        //         world.data.state = GameState::Halted(HaltedState::Halt);
        //     }
        //     Command::Stop => {
        //         world.data.state = GameState::Stopped(StoppedState::Stop);
        //     }
        //     _ => {}
        // }
        //
        // let mut kicker_team = None;
        //
        // match &world.data.state {
        //     GameState::Halted(_) => {
        //     }
        //     GameState::Stopped(stopped_state) => {
        //         match stopped_state {
        //             StoppedState::Stop => {
        //                 // TODO: prepare kickoffs :(
        //                 match referee_command {
        //                     Command::ForceStart => {
        //                         world.data.state = GameState::Running(RunningState::Run);
        //                     }
        //                     Command::PrepareKickoffBlue => {
        //                         kicker_team = Some(TeamColor::Blue);
        //                         if world.team_color == TeamColor::Blue {
        //                             world.data.state = GameState::Stopped(StoppedState::PrepareKickoff);
        //                         } else {
        //                             world.data.state = GameState::Stopped(StoppedState::Stop);
        //                         }
        //                     }
        //                     Command::PrepareKickoffYellow => {
        //                         kicker_team = Some(TeamColor::Yellow);
        //                         if world.team_color == TeamColor::Yellow {
        //                             world.data.state = GameState::Stopped(StoppedState::PrepareKickoff);
        //                         } else {
        //                             world.data.state = GameState::Stopped(StoppedState::Stop);
        //                         }
        //                     }
        //                     Command::PreparePenaltyBlue => {}
        //                     Command::PreparePenaltyYellow => {}
        //                     Command::NormalStart => {
        //                         world.data.state = GameState::Running(RunningState::KickOff(kicker_team.unwrap_or(TeamColor::Blue))); // FIX THIS auto color under too
        //                     }
        //                     _ => {}
        //                 }
        //             }
        //             StoppedState::PrepareKickoff => {
        //                 if let Command::NormalStart = referee_command {
        //                     world.data.state = GameState::Running(RunningState::KickOff(kicker_team.unwrap_or(TeamColor::Blue)));
        //                 }
        //             }
        //             StoppedState::PreparePenalty => {
        //                 if let Command::NormalStart = referee_command {
        //                     world.data.state = GameState::Running(RunningState::KickOff(kicker_team.unwrap_or(TeamColor::Blue)));
        //                 }
        //             }
        //             StoppedState::BallPlacement => {
        //                 // TODO: what the fuck ?
        //                 // if let Command::Continue {
        //                 //     world.data.state = GameState::Running(RunningState::KickOff);
        //                 // }
        //             }
        //         }
        //     }
        //     GameState::Running(running_state) => {
        //         match running_state {
        //             RunningState::KickOff(_) => {
        //                 if let Some(ball) = &world.ball {
        //                     if ball.velocity.xy().norm() > 0.3 {
        //                         world.data.state = GameState::Running(RunningState::Run);
        //                     }
        //                 }
        //                 if let Some(chrono) = &self.chrono {
        //                     if chrono.elapsed() > std::time::Duration::from_secs(10) {
        //                         world.data.state = GameState::Running(RunningState::Run);
        //                     }
        //                 } else {
        //                     // start chrono
        //                     self.chrono = Some(Instant::now());
        //                 }
        //                 // if let Command::AfterXSecondsOrIfBallMoved {
        //                 //     world.data.state = GameState::Running(RunningState::Run);
        //                 // }
        //             }
        //             RunningState::FreeKick => {
        //                 // if let Command::AfterXSecondsOrIfBallMoved {
        //                 //     world.data.state = GameState::Running(RunningState::Run);
        //                 // }
        //             }
        //             _ => {}
        //         }
        //     }
        // }
        //
        dbg!(referee_command);
        dbg!(&world.data.state);
    }
}
