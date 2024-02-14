use nalgebra::Point2;
use serde::Serialize;

/// A line segment in 2D space, defined by two points.
///
/// Note that the `start` and `end` fields should have the same units of
/// measurement.
#[derive(Clone, Serialize, Debug)]
pub struct Line {
    /// The starting point of the line segment.
    pub start: Point2<f64>,
    /// The ending point of the line segment.
    pub end: Point2<f64>,
}


impl Line{
    pub fn new(start: Point2<f64>, end: Point2<f64>) -> Self {
        Self { 
            start,
            end
        }
    }
}