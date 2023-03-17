use chrono::{DateTime, Utc};
use nalgebra::{Point2, Vector2};
use serde::Serialize;
use std::collections::HashMap;

/// The `AllyInfo` struct represents the information related to allies in the game.
#[derive(Serialize, Clone, Default, Debug)]
pub struct AllyInfo;

/// The `EnemyInfo` struct represents the information related to enemies in the game.
#[derive(Serialize, Clone, Default, Debug)]
pub struct EnemyInfo;

/// The `RobotVelocity` struct represents the velocity of a robot in the SSL.
#[derive(Serialize, Default, Debug, Clone, Copy)]
pub struct RobotVelocity {
    /// The linear velocity of the robot in meters per second.
    pub linear: Vector2<f64>,
    /// The angular velocity of the robot in radians per second.
    pub angular: f64,
}

/// The `RobotAcceleration` struct represents the acceleration of a robot in the SSL.
#[derive(Serialize, Default, Debug, Clone, Copy)]
pub struct RobotAcceleration {
    /// The linear acceleration of the robot in meters per second squared.
    pub linear: Vector2<f64>,
    /// The angular acceleration of the robot in radians per second squared.
    pub angular: f64,
}

/// The `Pose` struct represents the pose of a robot in the SSL, containing its position and orientation.
#[derive(Serialize, Default, Debug, Clone, Copy)]
pub struct Pose {
    /// The x-coordinate of the robot's position in 2D space (meters), with respect to the center of the field.
    pub x: f64,
    /// The y-coordinate of the robot's position in 2D space (meters), with respect to the center of the field.
    pub y: f64,
    /// The orientation of the robot in radians, measured counter-clockwise from the positive x-axis.
    pub orientation: f64,
}


impl Pose {
    pub fn new(x: f64, y: f64, orientation: f64) -> Pose {
        Pose { orientation, x, y }
    }
}

/// The `RobotMap` type is a hashmap that maps a robot ID to a Robot struct.
pub type RobotMap<T> = HashMap<u8, Robot<T>>;

/// The Robot struct represents a robot in the SSL game.
#[derive(Serialize, Default, Debug, Copy)]
pub struct Robot<T> {
    /// The unique identifier of the robot.
    pub id: u8,
    /// Whether or not the robot currently possesses the ball.
    pub has_ball: bool,
    /// Additional information about the robot (can be `AllyInfo` or `EnemyInfo`)
    pub robot_info: T,
    /// The current pose (x-coordinate, y-coordinate and orientation) of the robot.
    pub pose: Pose,
    /// The current velocity of the robot.
    pub velocity: RobotVelocity,
    /// The current acceleration of the robot.
    pub acceleration: RobotAcceleration,
    /// The timestamp indicating when this information was last updated.
    pub timestamp: DateTime<Utc>,
}

impl<T: Clone> Clone for Robot<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            has_ball: self.has_ball,
            robot_info: self.robot_info.clone(),
            pose: self.pose.clone(),
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.clone(),
            timestamp: self.timestamp,
        }
    }
}

impl<T> Robot<T> {
    /// Returns the position of the robot as a 2D point (x and y-coordinate).
    pub fn position(&self) -> Point2<f64> {
        Point2::new(self.pose.x, self.pose.y)
    }
}
