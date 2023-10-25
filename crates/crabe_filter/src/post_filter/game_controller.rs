use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::game_state::{
    GameState, HaltedState, RunningState, StoppedState,
};
use crabe_framework::data::world::{TeamColor, World};
use log::warn;
use nalgebra::Point2;

use crate::data::referee::event::{Event, GameEvent};
use crate::data::referee::RefereeCommand;
use std::time::Instant;

const FIRST_KICK_OFF: bool = false;
#[derive(Debug)]
pub struct GameControllerPostFilter {
    previous_game_event: Option<GameEvent>,
    previous_event: Option<Event>,
    previous_command: RefereeCommand,
    last_command: RefereeCommand,
    chrono: Option<Instant>, // TODO: Need to have an option here ?
    kicked_off_once: bool,
}

impl Default for GameControllerPostFilter {
    fn default() -> Self {
        GameControllerPostFilter {
            previous_game_event: None,
            previous_event: None,
            previous_command: RefereeCommand::Halt,
            last_command: RefereeCommand::Halt,
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
    fn fix_yourself(&mut self) {
        if self.chrono == None {
            self.chrono = Option::from(Instant::now());
        }
    }

    fn halt_state_branch(world: &mut World) {
        // Halt event, all robots should stop
        // consider that timeout is equivalent to normal halt
        world.data.state = GameState::Halted(HaltedState::Halt);
    }

    fn stop_state_branch(
        previous_game_event_opt: &Option<&GameEvent>,
        _previous_command: &RefereeCommand,
        world: &mut World,
        _kicked_off_once: &mut bool,
        mut _chrono: Option<Instant>,
    ) {
        
        if let Some(previous_game_event) = previous_game_event_opt {
            let previous_event = &previous_game_event.event;
            match previous_event {
                //Stopped events
                Event::Goal(goal_infos)=> {
                    // Goal has been scored, prepare for next kickoff phase
                    world.data.state = GameState::Stopped(StoppedState::PrepareKickoff(goal_infos.by_team.opposite()));
                }
                Event::BallLeftFieldTouchLine(left_field_infos) |
                Event::BallLeftFieldGoalLine(left_field_infos) => {
                    world.data.state = GameState::Stopped(StoppedState::BallPlacement(left_field_infos.by_team.opposite()));
                }
                Event::NoProgressInGame(_) |
                Event::AttackerDoubleTouchedBall(_) |
                Event::AimlessKick(_) |
                Event::DefenderInDefenseArea(_) |
                Event::KeeperHeldBall(_) |
                Event::BoundaryCrossing(_) |
                Event::BotDribbledBallTooFar(_) |
                Event::PenaltyKickFailed(_) |
                Event::AttackerTooCloseToDefenseArea(_) |
                Event::PlacementFailed(_) |
                Event::TooManyRobots(_) |
                Event::InvalidGoal(_) |
                Event::BotHeldBallDeliberately(_) |
                Event::UnsportingBehaviorMinor(_) |
                Event::UnsportingBehaviorMajor(_) |
                Event::BotPushedBot(_) => {
                    world.data.state = GameState::Stopped(StoppedState::Stop);
                }
                Event::BotTippedOver(infos) => {
                    // Precaution : stopping robots if there was an accident
                    // Robot has to be substituted
                    if infos.by_team.eq(&world.team_color) {
                        warn!("[IRL ACCIDENT] Robot {}", infos.by_bot.unwrap());
                    }
                    world.data.state = GameState::Halted(HaltedState::Halt);
                }
                
                //Non stopping fouls
                Event::AttackerTouchedBallInDefenseArea(_) => {
                    dbg!("AttackerTouchedBallInDefenseArea");
                }
                Event::BotKickedBallTooFast(_) => {
                    dbg!("BotKickedBallTooFast");
                }
                Event::BotCrashUnique(_) => {
                    dbg!("BotCrashUnique");
                }
                Event::BotCrashDrawn(_) |
                Event::DefenderTooCloseToKickPoint(_) |
                Event::BotTooFastInStop(_) |
                Event::BotInterferedPlacement(_) |
                Event::PossibleGoal(_) |
                Event::PlacementSucceeded(_) |
                Event::MultipleCards(_) |
                Event::MultipleFouls(_) |
                Event::BotSubstitution(_) |
                Event::ChallengeFlag(_) |
                Event::EmergencyStop(_) |
                Event::DeprecatedEvent => {
                    world.data.state = GameState::Stopped(StoppedState::Stop);
                }
            }
        } else {
            let team = if FIRST_KICK_OFF {world.team_color} else {world.team_color.opposite()};
            world.data.state = GameState::Stopped(StoppedState::PrepareKickoff(team));
        }
        _chrono = Some(Instant::now());
    }

    fn force_start_state_branch(
        _previous_event_opt: &Option<Event>,
        _previous_command: RefereeCommand,
        world: &mut World,
        mut _chrono: Option<Instant>,
    ) {
        world.data.state = GameState::Running(RunningState::Run);
    }

    fn normal_start_state_branch(
        previous_event_opt: &Option<Event>,
        previous_command: RefereeCommand,
        world: &mut World,
        mut _chrono: Option<Instant>,
    ) {
        _chrono = match _chrono{
            Some(_) => _chrono,
            None => Some(Instant::now())
        };
        if let Some(chrono) = _chrono {
            let ball_pos = match world.ball.clone() {
                None => {
                    Point2::new(0.0,0.0)
                }
                Some(ball) => {
                    ball.position.xy()
                }
            };
            if previous_command == RefereeCommand::PrepareKickoff(TeamColor::Blue){
                if chrono.elapsed() >= std::time::Duration::from_secs(10) {
                    world.data.state = GameState::Running(RunningState::Run);
                } else {
                    if ball_pos.x.abs()>0.5 || ball_pos.y.abs()>0.5 {
                        world.data.state = GameState::Running(RunningState::Run);
                    } else if world.data.state != GameState::Running(RunningState::Run){
                        world.data.state = GameState::Running(RunningState::KickOff(TeamColor::Blue));
                    }
                }
                return
            }
            else if previous_command == RefereeCommand::PrepareKickoff(TeamColor::Yellow){
                if chrono.elapsed() >= std::time::Duration::from_secs(10) {
                    world.data.state = GameState::Running(RunningState::Run);
                } else {
                    if ball_pos.x.abs()>0.5 || ball_pos.y.abs()>0.5 {
                        world.data.state = GameState::Running(RunningState::Run);
                    } else if world.data.state != GameState::Running(RunningState::Run){
                        world.data.state = GameState::Running(RunningState::KickOff(TeamColor::Yellow));
                    }
                }
                return
            }
        }
        else if let Some(previous_event) = previous_event_opt {
            match previous_event {
                Event::Goal(g) => {
                    // Kickoff is in progress by a team, place accordingly on your side
                    // 10s until we go into normal state
                    if let Some(chrono) = _chrono {
                        println!("Kickoff in progress ! It lasts for 10s at most");
                        if chrono.elapsed() > std::time::Duration::from_secs(10) {
                            world.data.state =
                                GameState::Running(RunningState::KickOff(g.by_team.opposite()));
                        }
                        return
                    } else {
                        // start chrono
                        _chrono = Some(Instant::now());
                    }
                }

                &_ => {}
            }
        } 
    }

    fn timeout_branch(world: &mut World, _team:TeamColor) {
        world.data.state = GameState::Halted(HaltedState::Timeout);
    }

    fn freekick_branch(world: &mut World, mut _chrono_opt: Option<Instant>, team:TeamColor) {
        if let Some(chrono) = _chrono_opt {
            if chrono.elapsed() > std::time::Duration::from_secs(10) {
                // if 10s have passed, game runs normally
                world.data.state = GameState::Running(RunningState::Run);
                _chrono_opt = Some(Instant::now());
            } else {
                // otherwise, we are still in the FreeKick state
                world.data.state = GameState::Running(RunningState::FreeKick(team));
            }
        }
    }

    fn prepare_penalty_branch(world: &mut World, mut _chrono_opt: Option<Instant>, team:TeamColor) {
        //TODO : the penalty comportement is complex, maybe we're missing a penalty RunningState
        
        if let Some(chrono) = _chrono_opt {
            if chrono.elapsed() >= std::time::Duration::from_secs(10) {
                world.data.state = GameState::Running(RunningState::Penalty(team));
            }
            return
        } else {
            world.data.state = GameState::Stopped(StoppedState::PreparePenalty(team));
            _chrono_opt = Some(Instant::now());
        }
    }

    fn prepare_kickoff_branch(world: &mut World, mut _chrono_opt: Option<Instant>, team: TeamColor) {
        world.data.state = GameState::Stopped(StoppedState::PrepareKickoff(team));
    }

    fn ball_placement_branch(world: &mut World,game_controller: &mut GameControllerPostFilter, team:TeamColor) {
        if let Some(chrono) = game_controller.chrono {
            // [ALLEMAGNE] chrono check peut être enlevé si pas de ball placement auto
            if chrono.elapsed() >= std::time::Duration::from_secs(30) {
                //TODO : when ball placement isn't done in 30 sec, what happen ?
                world.data.state = GameState::Stopped(StoppedState::BallPlacement(team.opposite()));
                //world.data.state = GameState::Running(RunningState::Run);
            } else {
                world.data.state = GameState::Stopped(StoppedState::BallPlacement(team));
            }
        }
        //reseting the chrono
        game_controller.fix_yourself();
    }
}

impl PostFilter for GameControllerPostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        self.fix_yourself();
        // grab data
        let last_referee_packet = match filter_data.referee.last() {
            None => {
                return;
            }
            Some(r) => r,
        };

        let ref_command = last_referee_packet.command.clone();
        //TODO : not sure about the indirect free refCommand
        //dbg!(&ref_command);
        match ref_command {
            RefereeCommand::Halt => GameControllerPostFilter::halt_state_branch(world),
            RefereeCommand::Deprecated |
            RefereeCommand::IndirectFree(_) |
            RefereeCommand::Stop => {
                GameControllerPostFilter::stop_state_branch(&last_referee_packet.game_events.last(), &self.previous_command, world, &mut self.kicked_off_once, self.chrono,)
            },
            RefereeCommand::NormalStart => {
                GameControllerPostFilter::normal_start_state_branch(&self.previous_event, self.previous_command.clone(), world, self.chrono,)
            },
            RefereeCommand::ForceStart => {
                GameControllerPostFilter::force_start_state_branch(&self.previous_event, self.previous_command.clone(), world, self.chrono,)
            },
            RefereeCommand::Timeout(team) => {
                GameControllerPostFilter::timeout_branch(world, team)
            }
            RefereeCommand::DirectFree(team) => {
                GameControllerPostFilter::freekick_branch(world, self.chrono, team)
            }
            RefereeCommand::BallPlacement(team) => {
                GameControllerPostFilter::ball_placement_branch(world, self, team)
            }
            RefereeCommand::PreparePenalty(team) => {
                GameControllerPostFilter::prepare_penalty_branch(world, self.chrono, team)
            }
            RefereeCommand::Goal(team) => {//TODO : It's recommended to use the score field from the team infos instead for goal detection and revoked goals.
                GameControllerPostFilter::prepare_kickoff_branch(world, self.chrono, team.opposite())
            }
            RefereeCommand::PrepareKickoff(team) => {
                GameControllerPostFilter::prepare_kickoff_branch(world, self.chrono, team)
            }
        }

        // Update previous gamestate & event
        if let Some(previous_game_event) = last_referee_packet.game_events.last() {
            self.previous_game_event = Option::from(previous_game_event.clone());
            self.previous_event = Option::from(previous_game_event.event.clone());
            //todo: don't clone this, specify lifetime
        }
        if ref_command != self.last_command{
            self.previous_command = self.last_command.clone();
            self.chrono = Option::from(Instant::now());
        }
        self.last_command = ref_command.clone();

        //update positive half team
        if let Some(team_on_positive_half) = last_referee_packet.positive_half {
            world.data.positive_half = team_on_positive_half
        }

        //TODO : ally and enemy
    }
}
