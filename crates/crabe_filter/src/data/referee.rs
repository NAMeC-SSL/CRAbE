use chrono::{DateTime, Duration, Utc};
use crabe_framework::data::world::TeamColor;
use event::GameEvent;
use nalgebra::Point2;

pub mod event;

/// MatchType is a meta information about the current match for easier log processing.
#[derive(
Clone,
Copy,
Debug
)]
#[repr(i32)]
pub enum MatchType {
    /// Not set
    UnknownMatch = 0,
    /// Match is part of the group phase
    GroupPhase = 1,
    /// Match is part of the elimination phase
    EliminationPhase = 2,
    /// Friendly match, not part of a tournament
    Friendly = 3,
}

/// The `Referee` struct contains various information about the referee of a match.
#[derive(Clone, Debug)]
pub struct Referee {
    /// A random UUID of the referee that is kept constant while running.
    pub source_identifier: Option<String>,
    /// Meta information about the current match that helps to process the logs after a competition.
    pub match_type: Option<MatchType>,
    /// The UNIX timestamp when the packet was sent, in microseconds.
    pub packet_timestamp: DateTime<Utc>,
    /// Represent the "coarse" stages of the match.
    pub stage: Stage,
    /// The number of microseconds left in the stage.
    /// Some stage have this value :
    /// - NORMAL_FIRST_HALF, NORMAL_HALF_TIME, NORMAL_SECOND_HALF
    /// - EXTRA_TIME_BREAK EXTRA_FIRST_HALF EXTRA_HALF_TIME EXTRA_SECOND_HALF
    /// - PENALTY_SHOOTOUT_BREAK
    /// If the stage runs over its specified time, this value becomes negative.
    pub stage_time_left: Option<Duration>,
    /// These are the "fine" states of play on the field..
    pub command: RefereeCommand,
    /// The number of commands issued since startup (mod 2^32).
    pub command_counter: u32,
    /// The UNIX timestamp when the command was issued, in microseconds.
    pub command_timestamp: DateTime<Utc>,
    /// Information about the ally team.
    pub ally: TeamInfo,
    /// Information about the enemy team.
    pub enemy: TeamInfo,
    /// The coordinates of the designated Position (in millimeters).
    /// Use only in the case of a ball placement command.
    pub designated_position: Option<Point2<f64>>,
    /// Information about the direction of play.
    /// True, if the blue team will have it's goal on the positive x-axis of the ssl-vision coordinate system.
    /// Obviously, the yellow team will play on the opposite half.
    pub positive_half: Option<TeamColor>,
    /// The command that will be issued after the current stoppage and ball placement to continue the game.
    pub next_command: Option<RefereeCommand>,
    /// All game events that were detected since the last RUNNING state.
    /// Will be cleared as soon as the game is continued.
    pub game_events: Vec<GameEvent>,
    /// All non-finished proposed game events that may be processed next.
    pub game_event_proposals: Vec<GameEventProposalGroup>,
    /// The time in microseconds that is remaining until the current action times out
    /// The time will not be reset. It can get negative.
    /// Possible actions where this time is relevant:
    /// - free kicks
    /// - kickoff, penalty kick, force start
    /// - ball placement
    pub current_action_time_remaining: Option<Duration>,
}

/// List of matching proposals.
#[derive(Clone, Debug)]
pub struct GameEventProposalGroup {
    /// The proposed game event.
    pub game_event: Vec<GameEvent>,
    /// Whether the proposal group was accepted.
    pub accepted: Option<bool>,
}
/// `Stage` represents different stages of a game.
#[derive(Clone, Debug)]
#[repr(i32)]
pub enum Stage {
    /// Indicates that the first half is about to start.
    NormalFirstHalfPre = 0,
    /// Indicates the first half of the normal game, before half time.
    NormalFirstHalf = 1,
    /// Indicates the half time period between the first and second halves.
    NormalHalfTime = 2,
    /// Indicates that the second half is about to start.
    NormalSecondHalfPre = 3,
    /// Indicates the second half of the normal game, after half time.
    NormalSecondHalf = 4,
    /// Represents the break before extra time.
    ExtraTimeBreak = 5,
    /// Indicates that the first half of the extra time is about to start.
    ExtraFirstHalfPre = 6,
    /// Represents the first half of the extra time.
    ExtraFirstHalf = 7,
    /// Indicates the half time period between the first and second extra halves.
    ExtraHalfTime = 8,
    /// Indicates that the second half of extra time is about to start.
    ExtraSecondHalfPre = 9,
    /// Represents the second half of the extra time.
    ExtraSecondHalf = 10,
    /// Represents the break period before the penalty shootout.
    PenaltyShootoutBreak = 11,
    /// Represents the penalty shootout stage.
    PenaltyShootout = 12,
    /// Indicates that the game has ended.
    PostGame = 13,
}

/// The `RefereeCommand` enum represents a set of possible commands that a referee can issue during a game.
/// These commands control the state of the game and dictate the actions that robots are allowed to perform at any given moment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RefereeCommand {
    /// Command indicating that all robots must halt movement immediately.
    Halt,
    /// Command instructing robots to maintain a distance of at least 50 cm from the ball.
    Stop,
    /// Command allowing a team to proceed with a prepared kickoff or penalty shot.
    NormalStart,
    /// Command declaring that the ball is freely accessible to any team, typically after a halt.
    ForceStart,
    /// Command allowing the specified team to move into position for a kickoff.
    PrepareKickoff(TeamColor),
    /// Command allowing the specified team to move into position for a penalty shot.
    PreparePenalty(TeamColor),
    /// Command allowing the specified team to take a direct free kick.
    DirectFree(TeamColor),
    /// Command allowing the specified team to take an indirect free kick.
    IndirectFree(TeamColor),
    /// Command indicating that the specified team has requested a timeout.
    Timeout(TeamColor),
    /// Deprecated: This command indicates the specified team has scored a goal.
    /// It's recommended to use the score field from the team infos instead for goal detection and revoked goals.
    Goal(TeamColor),
    /// Command equivalent to `Stop`, but the specified team must retrieve the ball and place it in a designated position.
    BallPlacement(TeamColor),
    ///Deprecated state
    Deprecated,
}

/// Information about a single team.
#[derive(Clone, Debug)]
pub struct TeamInfo {
    /// The team's name (empty string if operator has not typed anything).
    pub name: String,
    /// The number of goals scored by the team during normal play and overtime.
    pub score: u32,
    /// The number of red cards issued to the team since the beginning of the game.
    pub red_cards: u32,
    /// The amount of time (in microseconds) left on each yellow card issued to the team.
    /// If no yellow cards are issued, this array has no elements.
    /// Otherwise, times are ordered from smallest to largest.
    pub yellow_card_times: Vec<u32>,
    /// The total number of yellow cards ever issued to the team.
    pub yellow_cards: u32,
    /// The number of timeouts this team can still call.
    /// If in a timeout right now, that timeout is excluded.
    pub timeouts: u32,
    /// The number of microseconds of timeout this team can use.
    pub timeout_time: u32,
    /// The pattern number of this team's goalkeeper.
    pub goalkeeper: u32,
    /// The total number of countable fouls that act towards yellow cards
    pub foul_counter: Option<u32>,
    /// The number of consecutive ball placement failures of this team
    pub ball_placement_failures: Option<u32>,
    /// Indicate if the team is able and allowed to place the ball
    pub can_place_ball: Option<bool>,
    /// The maximum number of bots allowed on the field based on division and cards
    pub max_allowed_bots: Option<u32>,
    /// The team has submitted an intent to substitute one or more robots at the next chance
    pub bot_substitution_intent: Option<bool>,
    /// Indicate if the team reached the maximum allowed ball placement failures and is thus not allowed to place the ball anymore
    pub ball_placement_failures_reached: Option<bool>,
    /// The team is allowed to substitute one or more robots currently
    pub bot_substitution_allowed: Option<bool>,
}
