use std::time::Instant;
use chrono::{DateTime, Duration, Utc};
use nalgebra::Point2;
use crabe_framework::data::output::Command;
use crabe_framework::data::world::{Team, TeamColor};
use crate::data::event::GameEvent;

pub enum MatchType {
    UnknownMatch,
    GroupPhase,
    EliminationPhase,
    Friendly
}

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
    pub designated_position: Point2<f64>,
    pub positive_half: Option<TeamColor>,
    pub next_command: Option<Command>,
    pub game_events: Vec<GameEvent>,
    pub game_event_proposals: Vec<GameEventProposalGroup>,
    pub current_action_time_remaining: Option<Duration>
}

pub struct GameEventProposalGroup {
    pub game_event: Vec<GameEvent>,
    pub accepted: Option<bool>
}

enum Stage {
    NormalFirstHalfPre,
    NormalFirstHalf,
    NormalHalfTime,
    NormalSecondHalfPre,
    NormalSecondHalf,
    ExtraTimeBreak,
    ExtraFirstHalfPre,
    ExtraFirstHalf,
    ExtraHalfTime,
    ExtraSecondHalfPre,
    ExtraSecondHalf,
    PenaltyShootoutBreak,
    PenaltyShootout,
    PostGame
}

enum RefereeCommand {
    Halt,
    Stop,
    NormalStart,
    ForceStart,
    PrepareKickoff(TeamColor),
    PreparePenalty(TeamColor),
    DirectFree(TeamColor),
    Timeout(TeamColor),
    BallPlacement(TeamColor)
}