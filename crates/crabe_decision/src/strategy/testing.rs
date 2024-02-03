/// The `square` module contains a strategy that commands a robot to move in a square shape
/// in a counter-clockwise direction. It is used for testing purposes only and is not intended
/// for use in a game.
mod square;
mod avoid_obstacle;

pub use self::square::Square;
pub use self::avoid_obstacle::AvoidObstacle;