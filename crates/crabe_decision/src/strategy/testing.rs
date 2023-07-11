mod go_to_center;
/// The `square` module contains a strategy that commands a robot to move in a square shape
/// in a counter-clockwise direction. It is used for testing purposes only and is not intended
/// for use in a game.

mod square;

pub use self::go_to_center::GoToCenter;
pub use self::square::Square;

mod robots_formation;
mod go_to_position;

pub use self::robots_formation::RobotsFormation;
mod follow_ball;
pub use self::follow_ball::FollowBall;
mod circlerotation;
pub use self::circlerotation::CircleRotation;