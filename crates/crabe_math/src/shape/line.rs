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
    pub fn intersect(&self, line: &Line) -> bool{//not working correctly on edges points
        let a = self.start;
        let b = self.end;
        let c = line.start;
        let d = line.end;
    
        ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
    }
    pub fn intersection(&self, line: &Line) -> Option<Point2<f64>>{
        if !self.intersect(line){return None};
        let xdiff = (self.start.x - self.end.x, line.start.x - line.end.x);
        let ydiff = (self.start.y - self.end.y, line.start.y - line.end.y);
    
        fn det(a: (f64, f64), b: (f64, f64)) -> f64 {
            a.0 * b.1 - a.1 * b.0
        }
    
        let div = det(xdiff, ydiff);
        if div.abs() < 1e-6 {
            return None;
        }
    
        let d = (det((self.start.x, self.start.y), (self.end.x, self.end.y)), det((line.start.x, line.start.y), (line.end.x, line.end.y)));
        let x = det(d, xdiff) / div;
        let y = det(d, ydiff) / div;
        Some(Point2::new(x, y))
    }
    pub fn intersection_line(&self, line: &Line) -> Option<Point2<f64>>{
        let xdiff = (self.start.x - self.end.x, line.start.x - line.end.x);
        let ydiff = (self.start.y - self.end.y, line.start.y - line.end.y);
    
        fn det(a: (f64, f64), b: (f64, f64)) -> f64 {
            a.0 * b.1 - a.1 * b.0
        }
    
        let div = det(xdiff, ydiff);
        if div.abs() < 1e-6 {
            return None;
        }
    
        let d = (det((self.start.x, self.start.y), (self.end.x, self.end.y)), det((line.start.x, line.start.y), (line.end.x, line.end.y)));
        let x = det(d, xdiff) / div;
        let y = det(d, ydiff) / div;
        Some(Point2::new(x, y))
    }
    pub fn dist_to_point(&self, point: &Point2<f64>) -> f64{
        let line_direction = self.end - self.start;
        let point_direction = *point - self.start;
    
        let line_length_squared = line_direction.norm_squared();
        if line_length_squared == 0.0 {
            // The line segment has zero length, return distance to the start point.
            return point_direction.norm();
        }
    
        let t = point_direction.dot(&line_direction) / line_length_squared;
        if t < 0.0 {
            // The point is closest to the start of the line segment.
            return point_direction.norm();
        } else if t > 1.0 {
            // The point is closest to the end of the line segment.
            return (*point - self.end).norm();
        }
    
        // The point is closest to a point on the line segment.
        let closest_point = self.start + t * line_direction;
        (*point - closest_point).norm()
    }
    pub fn get_closest_point(&self, point: &Point2<f64>) -> Point2<f64> {
        let line_vec = self.end - self.start;
        let point_vec = *point - self.start;
        let line_length_squared = line_vec.magnitude_squared();
        let dot_product = line_vec.dot(&point_vec);
        
        let t = dot_product / line_length_squared;
        
        if t < 0. {
            return self.start;
        }else if t>1. {
            return self.end;
        }
        
        let closest_point = self.start + line_vec * t;
        
        closest_point
    }
}

fn ccw(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> bool {
    (c.y - a.y) * (b.x - a.x) >= (b.y - a.y) * (c.x - a.x)
}
