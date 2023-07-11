/// The `square` module contains a strategy that commands a robot to move in a square shape
/// in a counter-clockwise direction. It is used for testing purposes only and is not intended
/// for use in a game.
mod shooter;
pub use self::shooter::Shooter;
mod attacker;
pub use self::attacker::Attacker;
mod passer;
pub use self::passer::Passer;
mod receiver;
pub use self::receiver::Receiver;