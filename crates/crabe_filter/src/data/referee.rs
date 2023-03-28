use std::time::Instant;
use chrono::{DateTime, Duration, Utc};
use nalgebra::Point2;
use crabe_framework::data::output::Command;
use crabe_framework::data::world::{Team, TeamColor};
use crabe_framework::data::event::GameEvent;
use crabe_protocol::protobuf::game_controller_packet::MatchType;

#[derive(Debug)]
pub struct Referee {
    pub match_type: Option<MatchType>,
    pub packet_timestamp: DateTime<Utc>,
    pub stage: Stage,
    pub stage_time_left: Option<Duration>,
    pub command: RefereeCommand,
    pub command_counter: u32,
    pub command_timestamp: DateTime<Utc>,
    pub ally: Team,
    pub enemy: Team,
    pub designated_position: Option<Point2<f64>>,
    pub positive_half: Option<TeamColor>,
    pub next_command: Option<RefereeCommand>,
    pub game_events: Vec<GameEvent>,
    pub game_event_proposals: Vec<GameEventProposalGroup>,
    pub current_action_time_remaining: Option<Duration>
}

#[derive(Debug)]
pub struct GameEventProposalGroup {
    pub game_event: Vec<GameEvent>,
    pub accepted: Option<bool>
}

#[derive(Debug)]
#[repr(i32)]
pub enum Stage {
    NormalFirstHalfPre = 0,
    NormalFirstHalf = 1,
    NormalHalfTime = 2,
    NormalSecondHalfPre = 3,
    NormalSecondHalf = 4,
    ExtraTimeBreak = 5,
    ExtraFirstHalfPre = 6,
    ExtraFirstHalf = 7,
    ExtraHalfTime = 8,
    ExtraSecondHalfPre = 9,
    ExtraSecondHalf = 10,
    PenaltyShootoutBreak = 11,
    PenaltyShootout = 12,
    PostGame = 13
}

#[derive(Debug)]
pub enum RefereeCommand {
    Halt,
    Stop,
    NormalStart,
    ForceStart,
    PrepareKickoff(TeamColor),
    PreparePenalty(TeamColor),
    DirectFree(TeamColor),
    Timeout(TeamColor),
    BallPlacement(TeamColor),
    Deprecated
}