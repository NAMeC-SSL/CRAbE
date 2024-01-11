use chrono::Duration;
use crabe_framework::data::world::TeamColor;
use crabe_protocol::protobuf::game_controller_packet;
use crabe_protocol::protobuf::game_controller_packet::game_event::Type;
use nalgebra::Point2;

/// GameEvent contains exactly one game event.
/// Each game event has optional and required fields.
#[derive(Clone, Debug)]
pub struct GameEvent {
    /// Event type of this Game
    pub type_event: Option<GameEventType>,
    /// The origins of this game event.
    /// Empty, if it originates from game controller.
    pub origin: Vec<EventOrigin>,
    /// Unix timestamp in microseconds when the event was created.
    pub created_timestamp: Option<u64>,
    /// the event that occurred
    pub event: Event,
}

impl From<game_controller_packet::GameEvent> for GameEvent {
    fn from(packet: game_controller_packet::GameEvent) -> Self {
        Self {
            type_event: Option::from(GameEventType::from(packet.r#type())),
            origin: vec![],
            created_timestamp: None,
            event: Event::DeprecatedEvent,
        }
    }
}

/// All game event type.
/// See the protobuf message inside the crate `crabe_protocol` to see which game event is triggered by gc, auto_referee and human.
#[derive(Clone, Debug)]
pub enum GameEventType {
    Unknown,
    BallLeftFieldTouchLine,
    BallLeftFieldGoalLine,
    AimlessKick,
    AttackerTooCloseToDefenseArea,
    DefenderInDefenseArea,
    BoundaryCrossing,
    KeeperHeldBall,
    BotDribbledBallTooFar,
    BotPushedBot,
    BotHeldBallDeliberately,
    BotTippedOver,
    AttackerTouchedBallInDefenseArea,
    BotKickedBallTooFast,
    BotCrashUnique,
    BotCrashDrawn,
    DefenderTooCloseToKickPoint,
    BotTooFastInStop,
    BotInterferedPlacement,
    PossibleGoal,
    Goal,
    InvalidGoal,
    AttackerDoubleTouchedBall,
    PlacementSucceeded,
    PenaltyKickFailed,
    NoProgressInGame,
    PlacementFailed,
    MultipleCards,
    MultipleFouls,
    BotSubstitution,
    TooManyRobots,
    ChallengeFlag,
    ChallengeFlagHandled,
    EmergencyStop,
    UnsportingBehaviorMinor,
    UnsportingBehaviorMajor,
    Deprecated,
}

impl From<Type> for GameEventType {
    fn from(packet: Type) -> Self {
        match packet {
            Type::UnknownGameEventType => GameEventType::Unknown,
            Type::BallLeftFieldTouchLine => GameEventType::BallLeftFieldTouchLine,
            Type::BallLeftFieldGoalLine => GameEventType::BallLeftFieldGoalLine,
            Type::AimlessKick => GameEventType::AimlessKick,
            Type::AttackerTooCloseToDefenseArea => GameEventType::AttackerTooCloseToDefenseArea,
            Type::DefenderInDefenseArea => GameEventType::DefenderInDefenseArea,
            Type::BoundaryCrossing => GameEventType::BoundaryCrossing,
            Type::KeeperHeldBall => GameEventType::KeeperHeldBall,
            Type::BotDribbledBallTooFar => GameEventType::BotDribbledBallTooFar,
            Type::BotPushedBot => GameEventType::BotPushedBot,
            Type::BotHeldBallDeliberately => GameEventType::BotHeldBallDeliberately,
            Type::BotTippedOver => GameEventType::BotTippedOver,
            Type::AttackerTouchedBallInDefenseArea => {
                GameEventType::AttackerTouchedBallInDefenseArea
            }
            Type::BotKickedBallTooFast => GameEventType::BotKickedBallTooFast,
            Type::BotCrashUnique => GameEventType::BotCrashUnique,
            Type::BotCrashDrawn => GameEventType::BotCrashDrawn,
            Type::DefenderTooCloseToKickPoint => GameEventType::DefenderTooCloseToKickPoint,
            Type::BotTooFastInStop => GameEventType::BotTooFastInStop,
            Type::BotInterferedPlacement => GameEventType::BotInterferedPlacement,
            Type::PossibleGoal => GameEventType::PossibleGoal,
            Type::Goal => GameEventType::Goal,
            Type::InvalidGoal => GameEventType::InvalidGoal,
            Type::AttackerDoubleTouchedBall => GameEventType::AttackerDoubleTouchedBall,
            Type::PlacementSucceeded => GameEventType::PlacementSucceeded,
            Type::PenaltyKickFailed => GameEventType::PenaltyKickFailed,
            Type::NoProgressInGame => GameEventType::NoProgressInGame,
            Type::PlacementFailed => GameEventType::PlacementFailed,
            Type::MultipleCards => GameEventType::MultipleCards,
            Type::MultipleFouls => GameEventType::MultipleFouls,
            Type::BotSubstitution => GameEventType::BotSubstitution,
            Type::TooManyRobots => GameEventType::TooManyRobots,
            Type::ChallengeFlag => GameEventType::ChallengeFlag,
            Type::ChallengeFlagHandled => GameEventType::ChallengeFlagHandled,
            Type::EmergencyStop => GameEventType::EmergencyStop,
            Type::UnsportingBehaviorMinor => GameEventType::UnsportingBehaviorMinor,
            Type::UnsportingBehaviorMajor => GameEventType::UnsportingBehaviorMajor,
            Type::Prepared => GameEventType::Deprecated,
            Type::IndirectGoal => GameEventType::Deprecated,
            Type::ChippedGoal => GameEventType::Deprecated,
            Type::KickTimeout => GameEventType::Deprecated,
            Type::AttackerTouchedOpponentInDefenseArea => GameEventType::Deprecated,
            Type::AttackerTouchedOpponentInDefenseAreaSkipped => GameEventType::Deprecated,
            Type::BotCrashUniqueSkipped => GameEventType::Deprecated,
            Type::BotPushedBotSkipped => GameEventType::Deprecated,
            Type::DefenderInDefenseAreaPartially => GameEventType::Deprecated,
            Type::MultiplePlacementFailures => GameEventType::Deprecated,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    // Ball out of field events (Stopping)
    BallLeftFieldTouchLine(BallLeftField),
    BallLeftFieldGoalLine(BallLeftField),
    AimlessKick(AimlessKick),

    // Stopping fouls
    AttackerTooCloseToDefenseArea(AttackerTooCloseToDefenseArea),
    DefenderInDefenseArea(DefenderInDefenseArea),
    BoundaryCrossing(BoundaryCrossing),
    KeeperHeldBall(KeeperHeldBall),
    BotDribbledBallTooFar(BotDribbledBallTooFar),

    BotPushedBot(BotPushedBot),
    BotHeldBallDeliberately(BotHeldBallDeliberately),
    BotTippedOver(BotTippedOver),

    // Non-Stopping Fouls
    AttackerTouchedBallInDefenseArea(AttackerTouchedBallInDefenseArea),
    BotKickedBallTooFast(BotKickedBallTooFast),
    BotCrashUnique(BotCrashUnique),
    BotCrashDrawn(BotCrashDrawn),

    // Fouls while ball out of play
    DefenderTooCloseToKickPoint(DefenderTooCloseToKickPoint),
    BotTooFastInStop(BotTooFastInStop),
    BotInterferedPlacement(BotInterferedPlacement),

    // Scoring goals
    PossibleGoal(Goal),
    Goal(Goal),
    InvalidGoal(Goal),

    // Other events
    AttackerDoubleTouchedBall(AttackerDoubleTouchedBall),
    PlacementSucceeded(PlacementSucceeded),
    PenaltyKickFailed(PenaltyKickFailed),

    NoProgressInGame(NoProgressInGame),
    PlacementFailed(PlacementFailed),
    MultipleCards(TeamColor),
    MultipleFouls(MultipleFouls),
    TooManyRobots(TooManyRobots),
    BotSubstitution(TeamColor),
    ChallengeFlag(TeamColor),
    EmergencyStop(TeamColor),
    UnsportingBehaviorMinor(UnsportingBehaviorMinor),
    UnsportingBehaviorMajor(UnsportingBehaviorMajor),

    DeprecatedEvent,
}

//////////////////////////////////////////////////////
//               Event Struct Type                  //
//////////////////////////////////////////////////////

/// Represents an event where the ball left the field normally.
#[derive(Clone, Debug)]
pub struct BallLeftField {
    /// The team that last touched the ball.
    pub by_team: TeamColor,
    /// The bot that last touched the ball.
    pub by_bot: Option<u32>,
    /// The location where the ball left the field (in meter).
    pub location: Option<Point2<f64>>,
}

/// Represents an event where the ball left the field via goal line and a team committed an aimless kick.
#[derive(Clone, Debug)]
pub struct AimlessKick {
    /// The team that last touched the ball
    pub by_team: TeamColor,
    /// The bot that last touched the ball.
    pub by_bot: Option<u32>,
    /// The location where the ball left the field (in meter).
    pub location: Option<Point2<f64>>,
    /// The location where the ball was last touched (in meter).
    pub kick_location: Option<Point2<f64>>,
}

/// Represents an event where an attacking robot is located too close to the opponent's defense area during a stoppage or free kick.
#[derive(Clone, Debug)]
pub struct AttackerTooCloseToDefenseArea {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that is too close to the defense area.
    pub by_bot: Option<u32>,
    /// The location of the bot (in meter).
    pub location: Option<Point2<f64>>,
    /// The distance of the bot to the penalty area (in meter).
    pub distance: Option<f64>,
    /// The location of the ball at the moment when this foul occurred (in meter).
    pub ball_location: Option<Point2<f64>>,
}

/// Represents an event where a defender other than the keeper was fully located inside its own defense and touched the ball.
#[derive(Clone, Debug)]
pub struct DefenderInDefenseArea {
    /// The team that found guilty
    pub by_team: TeamColor,
    /// The bot that is inside the penalty area
    pub by_bot: Option<u32>,
    /// The location of the bot (in meter).
    pub location: Option<Point2<f64>>,
    /// The distance from bot case to the nearest point outside the defense area (in meter).
    pub distance: Option<f64>,
}

/// Represents an event where a robot chipped the ball over the field boundary out of the playing surface.
#[derive(Clone, Debug)]
pub struct BoundaryCrossing {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The location of the ball (in meter).
    pub location: Option<Point2<f64>>,
}

/// Represents an event where a keeper held the ball in its defense area for too long.
#[derive(Clone, Debug)]
pub struct KeeperHeldBall {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The location of the ball (in meter).
    pub location: Option<Point2<f64>>,
    /// The duration that the keeper hold the ball (in seconds).
    pub duration: Option<Duration>,
}

/// Represents an event where a bot dribbled to ball too far.
#[derive(Clone, Debug)]
pub struct BotDribbledBallTooFar {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that dribbled too far.
    pub by_bot: Option<u32>,
    /// The location where the dribbling started (in meter).
    pub start: Option<Point2<f64>>,
    /// The location where the maximum dribbling distance was reached (in meter).
    pub end: Option<Point2<f64>>,
}

/// Represents an event where a bot pushed another bot over a significant distance.
#[derive(Clone, Debug)]
pub struct BotPushedBot {
    /// The team that pushed the other team.
    pub by_team: TeamColor,
    /// The bot that pushed the other bot.
    pub violator: Option<u32>,
    /// The bot of the opposite team that was pushed.
    pub victim: Option<u32>,
    /// The location of the push (center between both bots) (in meter).
    pub location: Option<Point2<f64>>,
    /// The pushed distance (in meter).
    pub pushed_distance: Option<f64>,
}

/// Represents an event where a bot held the ball for too long.
#[derive(Clone, Debug)]
pub struct BotHeldBallDeliberately {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that holds the ball.
    pub by_bot: Option<u32>,
    /// The location of the ball (in meter).
    pub location: Option<Point2<f64>>,
    /// The duration that the bot hold the ball (in seconds).
    pub duration: Option<Duration>,
}

/// Represents an event where a bot tipped over.
#[derive(Clone, Debug)]
pub struct BotTippedOver {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that tipped over.
    pub by_bot: Option<u32>,
    /// The location of the bot (in meter).
    pub location: Option<Point2<f64>>,
    /// The location of the ball at the moment when this foul occurred (in meter).
    pub ball_location: Option<Point2<f64>>,
}

/// Represents an event where an attacker touched the ball inside the opponent defense area.
#[derive(Clone, Debug)]
pub struct AttackerTouchedBallInDefenseArea {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that is inside the penalty area.
    pub by_bot: Option<u32>,
    /// The location of the bot (in meter).
    pub location: Option<Point2<f64>>,
    /// The distance that the bot is inside the penalty area (in meter).
    pub distance: Option<f64>,
}

/// Represents an event where a bot kicked the ball too fast.
#[derive(Clone, Debug)]
pub struct BotKickedBallTooFast {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that kicked too fast.
    pub by_bot: Option<u32>,
    /// The location of the ball at the time of the highest speed (in meter).
    pub location: Option<Point2<f64>>,
    /// The absolute initial ball speed (kick speed) (in meter per second).
    pub initial_ball_speed: Option<f64>,
    /// Was the ball chipped?
    pub chipped: Option<bool>,
}

/// Represents an event where two robots crashed into each other and one team was found guilty to due significant speed difference.
#[derive(Clone, Debug)]
pub struct BotCrashUnique {
    /// The team that caused the crash.
    pub by_team: TeamColor,
    /// The bot that caused the crash.
    pub violator: Option<u32>,
    /// The bot of the opposite team that was involved in the crash.
    pub victim: Option<u32>,
    /// The location of the crash (center between both bots) (in meter).
    pub location: Option<Point2<f64>>,
    /// The calculated crash speed vector of the two bots (in meter per second).
    pub crash_speed: Option<f64>,
    /// The difference of the velocity of the two bots (in meter per second).
    pub speed_diff: Option<f64>,
    /// the angle [rad] in the range [0, π] of the bot velocity vectors
    /// an angle of 0 rad (  0°) means, the bots barely touched each other
    /// an angle of π rad (180°) means, the bots crashed frontal into each other
    pub crash_angle: Option<f64>,
}
/// Represents an event where two robots crashed into each other with similar speeds
#[derive(Clone, Debug)]
pub struct BotCrashDrawn {
    /// The bot of the yellow team.
    pub bot_yellow: Option<u32>,
    /// The bot of the blue team.
    pub bot_blue: Option<u32>,
    /// The location of the crash (center between both bots) (in meter).
    pub location: Option<Point2<f64>>,
    /// The calculated crash speed of the two bots (in meter per second).
    pub crash_speed: Option<f64>,
    /// The difference of the velocity of the two bots (in meter per second).
    pub speed_diff: Option<f64>,
    /// the angle [rad] in the range [0, π] of the bot velocity vectors
    /// an angle of 0 rad (  0°) means, the bots barely touched each other
    /// an angle of π rad (180°) means, the bots crashed frontal into each other.
    pub crash_angle: Option<f64>,
}

/// Represents an event where a bot of the defending team got too close to the kick point during a free kick.
#[derive(Clone, Debug)]
pub struct DefenderTooCloseToKickPoint {
    /// The team that was found guilty.
    pub by_team: TeamColor,
    /// The bot that violates the distance to the kick point.
    pub by_bot: Option<u32>,
    /// The location of the bot (in meter).
    pub location: Option<Point2<f64>>,
    /// The distance from bot to the kick point (including the minimum radius) (in meter).
    pub distance: Option<f64>,
}

/// Represents an event where a bot moved too fast while the game was stopped.
#[derive(Clone, Debug)]
pub struct BotTooFastInStop {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that was too fast.
    pub by_bot: Option<u32>,
    /// The location of the bot (in meter).
    pub location: Option<Point2<f64>>,
    /// The bot speed (in meter per second).
    pub speed: Option<f64>,
}

/// Represents an event where a bot interfered the ball placement of the other team.
#[derive(Clone, Debug)]
pub struct BotInterferedPlacement {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that interfered the placement.
    pub by_bot: Option<u32>,
    /// The location of the bot (in meter).
    pub location: Option<Point2<f64>>,
}

/// Represents an event where a team shot a goal
#[derive(Clone, Debug)]
pub struct Goal {
    /// The team that scored the goal.
    pub by_team: TeamColor,
    /// The team that shot the goal (different from by_team for own goals).
    pub kicking_team: Option<TeamColor>,
    /// The bot that shot the goal.
    pub kicking_bot: Option<u32>,
    /// The location where the ball entered the goal (in meter).
    pub location: Option<Point2<f64>>,
    /// The location where the ball was kicked (for deciding if this was a valid goal) (in meter).
    pub kick_location: Option<Point2<f64>>,
    /// The maximum height the ball reached during the goal kick (for deciding if this was a valid goal) (in meter).
    pub max_ball_height: Option<f64>,
    /// Number of robots of scoring team when the ball entered the goal (for deciding if this was a valid goal).
    pub num_bots_by_team: Option<u32>,
    /// The UNIX timestamp when the scoring team last touched the ball (in microsecond).
    pub last_touch_by_team: Option<u64>,
    /// An additional message with e.g. a reason for invalid goals.
    pub message: Option<String>,
}

/// Represents an event where an attacker touched the ball multiple times when it was not allowed to.
#[derive(Clone, Debug)]
pub struct AttackerDoubleTouchedBall {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that touched the ball twice.
    pub by_bot: Option<u32>,
    /// The location of the ball when it was first touched (in meter).
    pub location: Option<Point2<f64>>,
}

/// Represents an event where a team successfully placed the ball.
#[derive(Clone, Debug)]
pub struct PlacementSucceeded {
    /// The team that did the placement.
    pub by_team: TeamColor,
    /// The time taken for placing the ball (in second).
    pub time_taken: Option<f64>,
    /// The distance between placement location and actual ball position (in meter).
    pub precision: Option<f64>,
    /// The distance between the initial ball location and the placement position (in meter).
    pub distance: Option<f64>,
}

/// Represents an event where the penalty kick failed (by time or by keeper).
#[derive(Clone, Debug)]
pub struct PenaltyKickFailed {
    /// The team that last touched the ball.
    pub by_team: TeamColor,
    /// The location of the ball at the moment of this event (in minute).
    pub location: Option<Point2<f64>>,
    /// An explanation of the failure.
    pub reason: Option<String>,
}

/// Represents an event where game was stuck.
#[derive(Clone, Debug)]
pub struct NoProgressInGame {
    /// The location of the ball.
    pub location: Option<Point2<f64>>,
    /// The time that was waited (in second).
    pub time: Option<Duration>,
}

/// Represents an event where the ball placement failed.
#[derive(Clone, Debug)]
pub struct PlacementFailed {
    /// The team that failed.
    pub by_team: TeamColor,
    /// The remaining distance from ball to placement position (in meter).
    pub remaining_distance: Option<f64>,
}

/// Represents an event where a team collected multiple fouls, which results in a yellow card.
#[derive(Clone, Debug)]
pub struct MultipleFouls {
    /// The team that collected multiple fouls.
    pub by_team: TeamColor,
    /// The list of game events that caused the multiple fouls.
    pub caused_game_events: Vec<GameEvent>,
}

/// Represents an event where a team has too many robots on the field.
#[derive(Clone, Debug)]
pub struct TooManyRobots {
    /// The team that has too many robots.
    pub by_team: TeamColor,
    /// Number of robots allowed at the moment.
    pub num_robots_allowed: Option<u32>,
    /// Number of robots currently on the field.
    pub num_robots_on_field: Option<u32>,
    /// The location of the ball at the moment when this foul occurred (in meter).
    pub ball_location: Option<Point2<f64>>,
}

/// Represents an event where a team was found guilty for minor unsporting behavior.
#[derive(Clone, Debug)]
pub struct UnsportingBehaviorMinor {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// An explanation of the situation and decision.
    pub reason: String,
}

/// Represents an event where a team was found guilty for minor unsporting behavior.
#[derive(Clone, Debug)]
pub struct UnsportingBehaviorMajor {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// An explanation of the situation and decision.
    pub reason: String,
}

/// Enum that represent the origin of the event.
#[derive(Clone, Debug)]
pub enum EventOrigin {
    GameController,
    Autorefs(Vec<String>),
}
