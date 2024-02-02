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

    // return the intersection point between two lines (lines, not infinite long lines)
    // from https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection equations
    pub fn intersection_line(&self, line: &Line) -> Option<Point2<f64>>{
        let t_nominator = ((self.start.x - line.start.x)*(line.start.y - line.end.y)) - ((self.start.y - line.start.y) * (line.start.x - line.end.x));
        let u_nominator = -((self.start.x - self.end.x)*(self.start.y - line.start.y)) - ((self.start.y - self.end.y) * (self.start.x - line.start.x));

        let denominator = ((self.start.x - self.end.x)*(line.start.y - line.end.y)) - ((self.start.y - self.end.y) * (line.start.x - line.end.x));
        if denominator == 0.  {return None;}
        
        let t = t_nominator / denominator;
        let u = u_nominator / denominator;
        if t > 1. || t < 0. || u > 1. || u < 0. {return None;}
        return Some(self.point_allong_line(t));
    }

    // takes a number between 0 and 1 and return the point along the line situated at the specified dist
    // for example with 0. it will return the starting point, and with 0.5 it will return the center of the line
    pub fn point_allong_line(&self, x: f64) -> Point2<f64> {
        let p_x = self.start.x + x * (self.end.x - self.start.x);
        let p_y = self.start.y + x * (self.end.y - self.start.y);
        return Point2::new(p_x, p_y);
    }

    pub fn middle(&self) -> Point2<f64>{
        return self.point_allong_line(0.5);
    }

    pub fn norm(&self) -> f64{
        return ((self.start.x - self.end.x)*(self.start.x - self.end.x) +(self.start.y - self.end.y)*(self.start.y - self.end.y)).sqrt();             
    }
}
