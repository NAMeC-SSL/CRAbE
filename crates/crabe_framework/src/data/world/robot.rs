use chrono::{DateTime, Utc};
use nalgebra::{Point2, Vector2};
use serde::Serialize;
use std::collections::HashMap;

/// The `AllyInfo` struct represents the information related to allies in the game.
#[derive(Serialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllyInfo;

/// The `EnemyInfo` struct represents the information related to enemies in the game.
#[derive(Serialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnemyInfo;

/// The `RobotVelocity` struct represents the velocity of a robot in the SSL.
#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RobotVelocity {
    /// The linear velocity of the robot in meters per second.
    pub linear: Vector2<f64>,
    /// The angular velocity of the robot in radians per second.
    pub angular: f64,
}

/// The `RobotAcceleration` struct represents the acceleration of a robot in the SSL.
#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RobotAcceleration {
    /// The linear acceleration of the robot in meters per second squared.
    pub linear: Vector2<f64>,
    /// The angular acceleration of the robot in radians per second squared.
    pub angular: f64,
}

/// The `Pose` struct represents the pose of a robot in the SSL, containing its position and orientation.
#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pose {
    /// The orientation of the robot in radians.
    pub orientation: f64,
    /// The position in 2D of the robot in meters, with respect to the center of the field.
    pub position: Point2<f64>,
}

impl Pose {
    pub fn new(position: Point2<f64>, orientation: f64) -> Pose {
        Pose {
            orientation,
            position,
        }
    }
}

/// The `RobotMap` type is a hashmap that maps a robot ID to a Robot struct.
pub type RobotMap<T> = HashMap<u8, Robot<T>>;

/// The Robot struct represents a robot in the SSL game.
#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Robot<T> {
    /// The unique identifier of the robot.
    pub id: u8,
    /// Whether or not the robot currently possesses the ball.
    pub has_ball: bool,
    /// Additional information about the robot (can be `AllyInfo` or `EnemyInfo`)
    pub robot_info: T,
    /// The current pose (position and orientation) of the robot.
    pub pose: Pose,
    /// The current velocity of the robot.
    pub velocity: RobotVelocity,
    /// The current acceleration of the robot.
    pub acceleration: RobotAcceleration,
    /// The timestamp indicating when this information was last updated.
    pub timestamp: DateTime<Utc>,
    /// The position where the robot wants to go
    pub intended_pose: Pose,
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
            intended_pose: self.intended_pose.clone(),
        }
    }
}

impl<T> Robot<T> {
    pub fn distance(&self, point: &Point2<f64>) -> f64 {
        return distance(&self.pose.position, point)
    }
}

fn distance(p1: &Point2<f64>, p2: &Point2<f64>) -> f64 {
    (p2 - p1).norm()
}
