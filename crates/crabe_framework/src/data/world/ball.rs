use chrono::{DateTime, Utc};
use nalgebra::{Point2, Point3, Vector3};
use serde::Serialize;

/// The `Ball` struct represents the ball in the SSL game.
#[derive(Serialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ball {
    /// The position of the ball in 3D space in meters, with respect to the center of the field.
    pub position: Point3<f64>,
    /// The timestamp of when the data was captured.
    pub timestamp: DateTime<Utc>,
    /// The velocity of the ball in 3D space in meters per second.
    pub velocity: Vector3<f64>,
    /// The acceleration of the ball in 3D space in meters per second squared.
    pub acceleration: Vector3<f64>,
}

impl Ball {
    /// Returns the position of the ball as a 2D point (x and y-coordinate), with respect to the center of the field.
    pub fn position_2d(&self) -> Point2<f64> {
        Point2::new(self.position.x, self.position.y)
    }
}
