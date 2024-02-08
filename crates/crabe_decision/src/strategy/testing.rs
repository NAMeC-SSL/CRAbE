/// The `square` module contains a strategy that commands a robot to move in a square shape
/// in a counter-clockwise direction. It is used for testing purposes only and is not intended
/// for use in a game.
mod square;
pub use self::square::Square;
mod goto_ball;
pub use self::goto_ball::GotoBall;