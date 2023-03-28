use chrono::{DateTime, Duration};
use log::warn;
use nalgebra::Point2;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::{Team, TeamColor};
use crabe_protocol::protobuf::game_controller_packet::game_event as protocol_event;
use crabe_protocol::protobuf::game_controller_packet::game_event::{Event as ProtocolEventData, Goal as ProtocolGoal};

use crabe_protocol::protobuf::game_controller_packet::{GameEvent as ProtocolEvent, MatchType as ProtocolMatchType, Referee as ProtocolReferee, Vector2 as ProtocolVector2};
use crabe_protocol::protobuf::game_controller_packet::referee::{Command as ProtocolCommand, Command, Point as ProtocolPoint, Stage as ProtocolStage, TeamInfo};
use crabe_framework::data::event::{AimlessKick, AttackerDoubleTouchedBall, AttackerTooCloseToDefenseArea, AttackerTouchedBallInDefenseArea, BallLeftField, BotCrashDrawn, BotCrashUnique, BotDribbledBallTooFar, BotHeldBallDeliberately, BotInterferedPlacement, BotKickedBallTooFast, BotPushedBot, BotTippedOver, BotTooFastInStop, BoundaryCrossing, DefenderInDefenseArea, DefenderTooCloseToKickPoint, GameEvent, Goal, KeeperHeldBall, MultipleFouls, NoProgressInGame, PenaltyKickFailed, PlacementFailed, PlacementSucceeded, TooManyRobots, UnsportingBehavior};
use crate::data::FilterData;
use crabe_protocol::protobuf::game_controller_packet::Team as ProtocolTeam;
use crate::data;
use crate::data::referee::{GameEventProposalGroup, Referee, RefereeCommand, Stage};
use crate::pre_filter::PreFilter;
use crate::pre_filter::common::{create_date_time};

pub struct GameControllerFilter;

impl GameControllerFilter {
    pub fn new() -> Self {
        Self
    }
}

fn map_team_color(team: ProtocolTeam) -> TeamColor {
    match team {
        ProtocolTeam::Blue | ProtocolTeam::Unknown => TeamColor::Blue, // TODO: Handle unknown?
        ProtocolTeam::Yellow => TeamColor::Yellow
    }
}

fn map_team_color_i32(value: i32) -> TeamColor {
    map_team_color(ProtocolTeam::from_i32(value).unwrap_or(ProtocolTeam::Unknown))
}

fn map_command(incoming: ProtocolCommand) -> RefereeCommand {
    match incoming {
        Command::Halt => {
            RefereeCommand::Halt
        }
        Command::Stop => {
            RefereeCommand::Stop
        }
        Command::NormalStart => {
            RefereeCommand::NormalStart
        }
        Command::ForceStart => {
            RefereeCommand::ForceStart
        }
        Command::PrepareKickoffYellow => {
            RefereeCommand::PrepareKickoff(TeamColor::Yellow)
        }
        Command::PrepareKickoffBlue => {
            RefereeCommand::PrepareKickoff(TeamColor::Blue)
        }
        Command::PreparePenaltyYellow => {
            RefereeCommand::PreparePenalty(TeamColor::Yellow)
        }
        Command::PreparePenaltyBlue => {
            RefereeCommand::PreparePenalty(TeamColor::Blue)
        }
        Command::DirectFreeYellow => {
            RefereeCommand::DirectFree(TeamColor::Yellow)
        }
        Command::DirectFreeBlue => {
            RefereeCommand::DirectFree(TeamColor::Blue)
        }
        Command::TimeoutYellow => {
            RefereeCommand::Timeout(TeamColor::Yellow)
        }
        Command::TimeoutBlue => {
            RefereeCommand::Timeout(TeamColor::Blue)
        }
        Command::BallPlacementYellow => {
            RefereeCommand::BallPlacement(TeamColor::Yellow)
        }
        Command::BallPlacementBlue => {
            RefereeCommand::BallPlacement(TeamColor::Blue)
        }
        Command::IndirectFreeYellow |
        Command::IndirectFreeBlue |
        Command::GoalYellow |
        Command::GoalBlue => {
            RefereeCommand::Deprecated
        }
    }
}

fn map_point(point: ProtocolPoint) -> Point2<f64> {
    Point2::new(point.x.into(), point.y.into())
}

fn map_vector_point(vector: ProtocolVector2) -> Point2<f64> {
    Point2::new(vector.x.into(), vector.y.into())
}

fn map_ball_left_field(value: protocol_event::BallLeftField) -> BallLeftField {
    BallLeftField {
        by_team: map_team_color_i32(value.by_team),
        by_bot: value.by_bot,
        location: value.location.map(map_vector_point),
    }
}

fn map_goal(goal: ProtocolGoal) -> Goal {
    Goal {
        by_team: map_team_color_i32(goal.by_team),
        kicking_team: goal.kicking_team.map(map_team_color_i32),
        kicking_bot: goal.kicking_bot,
        location: goal.location.map(map_vector_point),
        kick_location: goal.kick_location.map(map_vector_point),
        max_ball_height: goal.max_ball_height.map(|h| h as f64),
        num_bots_by_team: goal.num_robots_by_team,
        last_touch_by_team: goal.last_touch_by_team,
        message: goal.message,
    }
}

fn map_event(event: ProtocolEvent) -> Option<GameEvent> {
    return if let Some(event) = event.event {
        match event {
            ProtocolEventData::BallLeftFieldTouchLine(data) => {
                Some(GameEvent::BallLeftFieldTouchLine(map_ball_left_field(data)))
            }
            ProtocolEventData::BallLeftFieldGoalLine(data) => {
                Some(GameEvent::BallLeftFieldTouchLine(map_ball_left_field(data)))
            }
            ProtocolEventData::AimlessKick(data) => {
                Some(GameEvent::AimlessKick(AimlessKick {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                    kick_location: data.kick_location.map(map_vector_point),
                }))
            }
            ProtocolEventData::AttackerTooCloseToDefenseArea(data) => {
                Some(GameEvent::AttackerTooCloseToDefenseArea(AttackerTooCloseToDefenseArea {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                    distance: data.distance.map(|d| d as f64),
                    ball_location: data.ball_location.map(map_vector_point),
                }))
            }
            ProtocolEventData::DefenderInDefenseArea(data) => {
                Some(GameEvent::DefenderInDefenseArea(DefenderInDefenseArea {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                    distance: data.distance.map(|d| d as f64),
                }))
            }
            ProtocolEventData::BoundaryCrossing(data) => {
                Some(GameEvent::BoundaryCrossing(BoundaryCrossing {
                    by_team: map_team_color_i32(data.by_team),
                    location: data.location.map(map_vector_point),
                }))
            }
            ProtocolEventData::KeeperHeldBall(data) => {
                Some(GameEvent::KeeperHeldBall(KeeperHeldBall {
                    by_team: map_team_color_i32(data.by_team),
                    location: data.location.map(map_vector_point),
                    duration: data.duration.map(|d| Duration::seconds(d as i64)), // TODO: More precision?
                }))
            }
            ProtocolEventData::BotDribbledBallTooFar(data) => {
                Some(GameEvent::BotDribbledBallTooFar(BotDribbledBallTooFar {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    start: data.start.map(map_vector_point),
                    end: data.end.map(map_vector_point),
                }))
            }
            ProtocolEventData::BotPushedBot(data) => {
                Some(GameEvent::BotPushedBot(BotPushedBot {
                    by_team: map_team_color_i32(data.by_team),
                    violator: data.violator,
                    victim: data.victim,
                    location: data.location.map(map_vector_point),
                    pushed_distance: data.pushed_distance.map(|d| d as f64),
                }))
            }
            ProtocolEventData::BotHeldBallDeliberately(data) => {
                Some(GameEvent::BotHeldBallDeliberately(BotHeldBallDeliberately {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                    duration: data.duration.map(|d| Duration::seconds(d as i64)), // TODO: More precision?
                }))
            }
            ProtocolEventData::BotTippedOver(data) => {
                Some(GameEvent::BotTippedOver(BotTippedOver {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                    ball_location: data.ball_location.map(map_vector_point),
                }))
            }
            ProtocolEventData::AttackerTouchedBallInDefenseArea(data) => {
                Some(GameEvent::AttackerTouchedBallInDefenseArea(AttackerTouchedBallInDefenseArea {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                    distance: data.distance.map(|d| d as f64),
                }))
            }
            ProtocolEventData::BotKickedBallTooFast(data) => {
                Some(GameEvent::BotKickedBallTooFast(BotKickedBallTooFast {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                    initial_ball_speed: data.initial_ball_speed.map(|s| s as f64),
                    chipped: data.chipped,
                }))
            }
            ProtocolEventData::BotCrashUnique(data) => {
                Some(GameEvent::BotCrashUnique(BotCrashUnique {
                    by_team: map_team_color_i32(data.by_team),
                    violator: data.violator,
                    victim: data.victim,
                    location: data.location.map(map_vector_point),
                    crash_speed: data.crash_speed.map(|s| s as f64),
                    speed_diff: data.speed_diff.map(|d| d as f64),
                    crash_angle: data.crash_angle.map(|a| a as f64),
                }))
            }
            ProtocolEventData::BotCrashDrawn(data) => {
                Some(GameEvent::BotCrashDrawn(BotCrashDrawn {
                    bot_blue: data.bot_blue,
                    bot_yellow: data.bot_yellow,
                    crash_speed: data.crash_speed.map(|s| s as f64),
                    speed_diff: data.speed_diff.map(|d| d as f64),
                    crash_angle: data.crash_angle.map(|a| a as f64),
                }))
            }
            ProtocolEventData::BotTooFastInStop(data) => {
                Some(GameEvent::BotTooFastInStop(BotTooFastInStop {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                    speed: data.speed.map(|s| s as f64),
                }))
            }
            ProtocolEventData::BotInterferedPlacement(data) => {
                Some(GameEvent::BotInterferedPlacement(BotInterferedPlacement {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                }))
            }
            ProtocolEventData::PossibleGoal(data) => {
                Some(GameEvent::PossibleGoal(map_goal(data)))
            }
            ProtocolEventData::Goal(data) => {
                Some(GameEvent::Goal(map_goal(data)))
            }
            ProtocolEventData::InvalidGoal(data) => {
                Some(GameEvent::InvalidGoal(map_goal(data)))
            }
            ProtocolEventData::AttackerDoubleTouchedBall(data) => {
                Some(GameEvent::AttackerDoubleTouchedBall(AttackerDoubleTouchedBall {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                }))
            }
            ProtocolEventData::PlacementSucceeded(data) => {
                Some(GameEvent::PlacementSucceeded(PlacementSucceeded {
                    by_team: map_team_color_i32(data.by_team),
                    time_taken: data.time_taken.map(|d| Duration::seconds(d as i64)),
                    precision: data.precision.map(|p| p as f64),
                    distance: data.distance.map(|d| d as f64),
                }))
            }
            ProtocolEventData::PlacementFailed(data) => {
                Some(GameEvent::PlacementFailed(PlacementFailed {
                    by_team: map_team_color_i32(data.by_team),
                    remaining_distance: data.remaining_distance.map(|d| d as f64),
                }))
            }
            ProtocolEventData::PenaltyKickFailed(data) => {
                Some(GameEvent::PenaltyKickFailed(PenaltyKickFailed {
                    by_team: map_team_color_i32(data.by_team),
                    location: data.location.map(map_vector_point),
                }))
            }
            ProtocolEventData::NoProgressInGame(data) => {
                Some(GameEvent::NoProgressInGame(NoProgressInGame {
                    location: data.location.map(map_vector_point),
                    time: data.time.map(|d| Duration::seconds(d as i64)),
                }))
            }
            ProtocolEventData::MultipleCards(data) => {
                Some(GameEvent::MultipleCards(map_team_color_i32(data.by_team)))
            }
            ProtocolEventData::MultipleFouls(data) => {
                Some(GameEvent::MultipleFouls(MultipleFouls {
                    by_team: map_team_color_i32(data.by_team),
                    caused_game_events: vec![], // TODO
                }))
            }
            ProtocolEventData::BotSubstitution(data) => {
                Some(GameEvent::BotSubstitution(map_team_color_i32(data.by_team)))
            }
            ProtocolEventData::TooManyRobots(data) => {
                Some(GameEvent::TooManyRobots(TooManyRobots {
                    by_team: map_team_color_i32(data.by_team),
                    num_robots_allowed: data.num_robots_allowed.map(|n| if n < 0 { 0 } else { n as u32 }),
                    num_robots_on_field: data.num_robots_on_field.map(|n| if n < 0 { 0 } else { n as u32 }),
                    ball_location: data.ball_location.map(map_vector_point),
                }))
            }
            ProtocolEventData::ChallengeFlag(data) => {
                Some(GameEvent::ChallengeFlag(map_team_color_i32(data.by_team)))
            }
            ProtocolEventData::EmergencyStop(data) => {
                Some(GameEvent::EmergencyStop(map_team_color_i32(data.by_team)))
            }
            ProtocolEventData::UnsportingBehaviorMinor(data) => {
                Some(GameEvent::UnsportingBehaviorMinor(UnsportingBehavior {
                    by_team: map_team_color_i32(data.by_team),
                    reason: data.reason,
                }))
            }
            ProtocolEventData::UnsportingBehaviorMajor(data) => {
                Some(GameEvent::UnsportingBehaviorMajor(UnsportingBehavior {
                    by_team: map_team_color_i32(data.by_team),
                    reason: data.reason,
                }))
            }
            ProtocolEventData::DefenderTooCloseToKickPoint(data) => {
                Some(GameEvent::DefenderTooCloseToKickPoint(DefenderTooCloseToKickPoint {
                    by_team: map_team_color_i32(data.by_team),
                    by_bot: data.by_bot,
                    location: data.location.map(map_vector_point),
                    distance: data.distance.map(|d| d as f64),
                }))
            }

            // DEPRECATED
            ProtocolEventData::Prepared(_) |
            ProtocolEventData::IndirectGoal(_) |
            ProtocolEventData::ChippedGoal(_) |
            ProtocolEventData::KickTimeout(_) |
            ProtocolEventData::AttackerTouchedOpponentInDefenseArea(_) |
            ProtocolEventData::AttackerTouchedOpponentInDefenseAreaSkipped(_) |
            ProtocolEventData::BotCrashUniqueSkipped(_) |
            ProtocolEventData::BotPushedBotSkipped(_) |
            ProtocolEventData::DefenderInDefenseAreaPartially(_) |
            ProtocolEventData::MultiplePlacementFailures(_) => {
                None
            }
        }
    } else {
        None
    };
}

struct RefereeDeserializationError;

fn map_stage(stage: ProtocolStage) -> Stage {
    match stage {
        ProtocolStage::NormalFirstHalfPre => { Stage::NormalFirstHalfPre }
        ProtocolStage::NormalFirstHalf => { Stage::NormalFirstHalf }
        ProtocolStage::NormalHalfTime => { Stage::NormalHalfTime }
        ProtocolStage::NormalSecondHalfPre => { Stage::NormalSecondHalfPre }
        ProtocolStage::NormalSecondHalf => { Stage::NormalSecondHalf }
        ProtocolStage::ExtraTimeBreak => { Stage::ExtraTimeBreak }
        ProtocolStage::ExtraFirstHalfPre => { Stage::ExtraFirstHalfPre }
        ProtocolStage::ExtraFirstHalf => { Stage::ExtraFirstHalf }
        ProtocolStage::ExtraHalfTime => { Stage::ExtraHalfTime }
        ProtocolStage::ExtraSecondHalfPre => { Stage::ExtraSecondHalfPre }
        ProtocolStage::ExtraSecondHalf => { Stage::ExtraSecondHalf }
        ProtocolStage::PenaltyShootoutBreak => { Stage::PenaltyShootoutBreak }
        ProtocolStage::PenaltyShootout => { Stage::PenaltyShootout }
        ProtocolStage::PostGame => { Stage::PostGame }
    }
}

fn map_referee(mut referee: ProtocolReferee, team_color: &TeamColor) -> Result<Referee, RefereeDeserializationError> {
    let (ally, enemy) = match team_color {
        TeamColor::Yellow => (referee.yellow, referee.blue),
        TeamColor::Blue => (referee.blue, referee.yellow)
    };

    Ok(
        Referee {
            match_type: referee.match_type.map(|m| ProtocolMatchType::from_i32(m)).flatten(),
            packet_timestamp: create_date_time(referee.packet_timestamp as i64),
            stage: ProtocolStage::from_i32(referee.stage).map(map_stage).ok_or(RefereeDeserializationError)?,
            stage_time_left: referee.stage_time_left.map(|d| Duration::microseconds(d as i64)),
            command: ProtocolCommand::from_i32(referee.command).map(map_command).ok_or(RefereeDeserializationError)?,
            command_counter: referee.command_counter,
            command_timestamp: create_date_time(referee.command_timestamp as i64),
            ally: Team {
                color: team_color.clone(),
                name: Some(ally.name),
            },
            enemy: Team {
                color: team_color.opposite(),
                name: Some(enemy.name),
            },
            designated_position: referee.designated_position.map(map_point),
            positive_half: referee.blue_team_on_positive_half.map(|b| if b { TeamColor::Blue } else { TeamColor::Yellow }),
            next_command: referee.next_command.map(|c| ProtocolCommand::from_i32(c)).flatten().map(map_command),
            game_events: referee.game_events.drain(..).filter_map(map_event).collect(),
            game_event_proposals: referee.game_event_proposals.drain(..).map(|mut p| GameEventProposalGroup {
                game_event: p.game_event.drain(..).filter_map(map_event).collect(),
                accepted: p.accepted,
            }).collect(),
            current_action_time_remaining: referee.current_action_time_remaining.map(|d| Duration::microseconds(d as i64)),
        }
    )
}

impl PreFilter for GameControllerFilter {
    fn step(
        &mut self,
        inbound_data: &mut InboundData,
        team_color: &TeamColor,
        filter_data: &mut FilterData,
    ) {
        filter_data.referee.extend(inbound_data.gc_packet.drain(..).filter_map(|p| map_referee(p, team_color).ok()));
    }
}