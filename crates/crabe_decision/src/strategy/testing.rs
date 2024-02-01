/// The `square` module contains a strategy that commands a robot to move in a square shape
/// in a counter-clockwise direction. It is used for testing purposes only and is not intended
/// for use in a game.
// mod square;
// pub use self::square::Square;
// mod go_to_middle;
// pub use self::go_to_middle::GoToMiddle;
mod go_to_four_corners;
pub use self::go_to_four_corners::GoToFourCorners;
