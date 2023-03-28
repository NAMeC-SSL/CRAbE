use chrono::Duration;
use nalgebra::Point2;
use crate::data::world::TeamColor;
use crabe_protocol::protobuf::game_controller_packet::referee::Point;
use crabe_protocol::protobuf::game_controller_packet::{game_event as protocol_event, Vector2 as ProtocolVector2};

#[derive(Debug)]
pub enum EventOrigin {
    GameController,
    Autorefs(Vec<String>)
}

#[derive(Debug)]
pub struct BallLeftField {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
}

#[derive(Debug)]
pub struct AimlessKick {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub kick_location: Option<Point2<f64>>,
}


#[derive(Debug)]
pub struct Goal {
    pub by_team: TeamColor,
    pub kicking_team: Option<TeamColor>,
    pub kicking_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub kick_location: Option<Point2<f64>>,
    pub max_ball_height: Option<f64>,
    pub num_bots_by_team: Option<u32>,
    pub last_touch_by_team: Option<u64>,
    pub message: Option<String>
}

#[derive(Debug)]
pub struct BotTooFastInStop {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub speed: Option<f64>
}

#[derive(Debug)]
pub struct DefenderTooCloseToKickPoint {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>
}

#[derive(Debug)]
pub struct BotCrashDrawn {
    pub bot_blue: Option<u32>,
    pub bot_yellow: Option<u32>,
    pub crash_speed: Option<f64>,
    pub speed_diff: Option<f64>,
    pub crash_angle: Option<f64>
}

#[derive(Debug)]
pub struct BotCrashUnique {
    pub by_team: TeamColor,
    pub violator: Option<u32>,
    pub victim: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub crash_speed: Option<f64>,
    pub speed_diff: Option<f64>,
    pub crash_angle: Option<f64>
}

#[derive(Debug)]
pub struct BotPushedBot {
    pub by_team: TeamColor,
    pub violator: Option<u32>,
    pub victim: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub pushed_distance: Option<f64>
}

#[derive(Debug)]
pub struct BotTippedOver {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub ball_location: Option<Point2<f64>>
}

#[derive(Debug)]
pub struct DefenderInDefenseArea {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>,
}

#[derive(Debug)]
pub struct DefenderInDefenseAreaPartially {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>,
    pub ball_location: Option<Point2<f64>>
}

#[derive(Debug)]
pub struct AttackerTouchedBallInDefenseArea {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>,
}

#[derive(Debug)]
pub struct BotKickedBallTooFast {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub initial_ball_speed: Option<f64>,
    pub chipped: Option<bool>,
}

#[derive(Debug)]
pub struct BotDribbledBallTooFar {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub start: Option<Point2<f64>>,
    pub end: Option<Point2<f64>>
}

#[derive(Debug)]
pub struct AttackerDoubleTouchedBall {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>
}

#[derive(Debug)]
pub struct AttackerTooCloseToDefenseArea {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>,
    pub ball_location: Option<Point2<f64>>,
}

#[derive(Debug)]
pub struct BotHeldBallDeliberately {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub duration: Option<Duration>,
}

#[derive(Debug)]
pub struct BotInterferedPlacement {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
}

#[derive(Debug)]
pub struct MultipleFouls {
    pub by_team: TeamColor,
    pub caused_game_events: Vec<GameEvent>,
}

#[derive(Debug)]
pub struct NoProgressInGame {
    pub location: Option<Point2<f64>>,
    pub time: Option<Duration>
}

#[derive(Debug)]
pub struct PlacementFailed {
    pub by_team: TeamColor,
    pub remaining_distance: Option<f64>,
}

#[derive(Debug)]
pub struct UnsportingBehavior {
    pub by_team: TeamColor,
    pub reason: String,
}

#[derive(Debug)]
pub struct KeeperHeldBall {
    pub by_team: TeamColor,
    pub location: Option<Point2<f64>>,
    pub duration: Option<Duration>
}

#[derive(Debug)]
pub struct PlacementSucceeded {
    pub by_team: TeamColor,
    pub time_taken: Option<Duration>,
    pub precision: Option<f64>,
    pub distance: Option<f64>
}

#[derive(Debug)]
pub struct TooManyRobots {
    pub by_team: TeamColor,
    pub num_robots_allowed: Option<u32>,
    pub num_robots_on_field: Option<u32>,
    pub ball_location: Option<Point2<f64>>
}

#[derive(Debug)]
pub struct BoundaryCrossing {
    pub by_team: TeamColor,
    pub location: Option<Point2<f64>>
}

#[derive(Debug)]
pub struct PenaltyKickFailed {
    pub by_team: TeamColor,
    pub location: Option<Point2<f64>>
}

#[derive(Debug)]
pub enum GameEvent {
    Unknown,
    BallLeftFieldTouchLine(BallLeftField),
    BallLeftFieldGoalLine(BallLeftField),
    AimlessKick(AimlessKick),
    AttackerTooCloseToDefenseArea(AttackerTooCloseToDefenseArea),
    DefenderInDefenseArea(DefenderInDefenseArea),
    BoundaryCrossing(BoundaryCrossing),
    KeeperHeldBall(KeeperHeldBall),
    BotDribbledBallTooFar(BotDribbledBallTooFar),
    BotPushedBot(BotPushedBot),
    BotHeldBallDeliberately(BotHeldBallDeliberately),
    BotTippedOver(BotTippedOver),
    AttackerTouchedBallInDefenseArea(AttackerTouchedBallInDefenseArea),
    BotKickedBallTooFast(BotKickedBallTooFast),
    BotCrashUnique(BotCrashUnique),
    BotCrashDrawn(BotCrashDrawn),
    DefenderTooCloseToKickPoint(DefenderTooCloseToKickPoint),
    BotTooFastInStop(BotTooFastInStop),
    BotInterferedPlacement(BotInterferedPlacement),
    PossibleGoal(Goal),
    Goal(Goal),
    InvalidGoal(Goal),
    AttackerDoubleTouchedBall(AttackerDoubleTouchedBall),
    PlacementSucceeded(PlacementSucceeded),
    PlacementFailed(PlacementFailed),
    PenaltyKickFailed(PenaltyKickFailed),
    NoProgressInGame(NoProgressInGame),
    MultipleCards(TeamColor),
    MultipleFouls(MultipleFouls),
    TooManyRobots(TooManyRobots),
    BotSubstitution(TeamColor),
    ChallengeFlag(TeamColor),
    EmergencyStop(TeamColor),
    UnsportingBehaviorMinor(UnsportingBehavior),
    UnsportingBehaviorMajor(UnsportingBehavior),
}