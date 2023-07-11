use nalgebra::Point2;
use serde::Serialize;

/// Represents a circle in 2D space defined by its center point and radius.
///
/// Note that the `center` and `radius` fields should have the same units of
/// measurement.
#[derive(Serialize, Clone, Debug)]
pub struct Circle {
    /// The center point of the circle.
    pub center: Point2<f64>,
    /// The radius of the circle.
    pub radius: f64,
}

impl Circle{
    pub fn new(center: Point2<f64>, radius: f64) -> Self {
        Self { 
            center,
            radius
        }
    }
}