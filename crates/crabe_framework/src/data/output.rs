use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The FeedbackMap type is a hash map that stores feedback data for robots in the game.
/// Each robot is identified by its ID.
pub type FeedbackMap = HashMap<u32, Feedback>;

/// The Feedback struct contains information about the feedback data for a robot in the game.
#[derive(Debug)]
pub struct Feedback {
    /// A boolean value indicating whether the robot has possession of the ball.
    pub has_ball: bool,
    /// The current voltage level of the robot (only in real).
    pub voltage: f32,
}

/// The CommandMap type is a hash map that stores commands to be sent to the robots in the game.
/// Each robot is identified by its ID.
pub type CommandMap = HashMap<u8, Command>;

/// The Kick enum is used to specify the type of kick to be performed by a robot.
#[derive(Copy, Clone, Deserialize)]
pub enum Kick {
    /// A straight kick with the specified power.
    StraightKick { power: f32 },
    /// A chip kick (lob) with the specified power.
    ChipKick { power: f32 },
}

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    /// Velocity forward in m.s-1 (towards the dribbler)
    pub forward_velocity: f32,
    /// Velocity to the left in m.s-1
    pub left_velocity: f32,
    /// Angular velocity rad.s-1 in (counter-clockwise)
    pub angular_velocity: f32,
    /// Order to charge the capacitor of the robot
    pub charge: bool,
    /// Order to kick the ball, if None doesn't KICK
    pub kick: Option<Kick>,
    /// Dribbler speed in rounds per minute rpm
    pub dribbler: f32,
}
