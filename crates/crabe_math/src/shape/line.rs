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
    pub fn intersect(&self, l: Line) -> Option<Point2<f64>>{
        let d = (self.start.x - l.start.y) * (self.end.x - self.start.x) - (l.end.x - l.start.x) * (self.end.y - self.start.y);
        
        if d == 0.{
            return None;
        }
        
        let u_a = ((self.start.x - l.start.x) * (l.start.y - l.end.y) - (self.start.y - l.start.y) * (l.start.x - l.end.x)) / d;
        let u_b = -((self.start.x - self.end.x) * (self.start.y - l.start.y) - (self.start.y - self.end.y) * (self.start.x - l.start.x)) / d;
        if !(u_a <= 1. && u_a >= 0. && u_b <= 1. && u_b >= 0.){
            return None;
        }
        
        let x = self.start.x + u_a * (self.end.x - self.start.x);
        let y = self.start.y + u_a * (self.end.y - self.start.y);
        
        Some(Point2::new(x, y))
    }

    pub fn new(start: Point2<f64>, end: Point2<f64>) -> Line{
        Line{start: start, end: end}
    }
}