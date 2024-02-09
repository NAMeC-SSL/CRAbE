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

impl Line {
    pub fn new(start: Point2<f64>, end: Point2<f64>) -> Self {
        Self { start, end }
    }

    // return the intersection point between two lines
    // (not working if center of line is 0)
    // from https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection equations
    pub fn intersection_line(&self, line: &Line) -> Option<Point2<f64>> {
        let x_nominator = (self.start.x * self.end.y - self.start.y * self.end.x)
            * (line.start.x - line.end.x)
            - (self.start.x - self.end.x) * (line.start.x * line.end.y - line.start.y * line.end.x);
        let y_nominator = (self.start.x * self.end.y - self.start.y * self.end.x)
            * (line.start.y - line.end.y)
            - (self.start.y - self.end.y) * (line.start.x * line.end.y - line.start.y * line.end.x);

        let denominator = ((self.start.x - self.end.x) * (line.start.y - line.end.y))
            - ((self.start.y - self.end.y) * (line.start.x - line.end.x));
        if denominator == 0. {
            println!("[line.rs] Lines are parrallel");
            return None;
        }

        let x = x_nominator / denominator;
        let y = y_nominator / denominator;
        return Some(Point2::new(x, y));
    }

    // return the intersection point between a segment and a line
    // from https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection equations
    pub fn intersection_segment_line(&self, line: &Line) -> Option<Point2<f64>> {
        if let Some(intersection) = self.intersection_line(line) {
            if let Some(_) = self.orthogonal_projection_point_on_segment(&intersection) {
                return Some(intersection);
            }
        }
        None
    }

    // return the intersection point between two segments
    // from https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection equations
    pub fn intersection_segment(&self, line: &Line) -> Option<Point2<f64>> {
        let t_nominator = ((self.start.x - line.start.x) * (line.start.y - line.end.y))
            - ((self.start.y - line.start.y) * (line.start.x - line.end.x));
        let u_nominator = -((self.start.x - self.end.x) * (self.start.y - line.start.y))
            - ((self.start.y - self.end.y) * (self.start.x - line.start.x));

        let denominator = ((self.start.x - self.end.x) * (line.start.y - line.end.y))
            - ((self.start.y - self.end.y) * (line.start.x - line.end.x));
        if denominator == 0. {
            return None;
        }

        let t = t_nominator / denominator;
        let u = u_nominator / denominator;
        if t > 1. || t < 0. || u > 1. || u < 0. {
            return None;
        }
        return Some(self.point_allong_line(t));
    }

    // return the closest point on the line
    pub fn closest_point_on_line(&self, point: &Point2<f64>) -> Point2<f64> {
        let line_direction = self.end - self.start;
        let point_direction = *point - self.start;

        let line_length_squared = line_direction.norm_squared();
        if line_length_squared == 0.0 {
            // The line segment has zero length, return the start point.
            return self.start;
        }
        let t = point_direction.dot(&line_direction) / line_length_squared;

        // The point is closest to a point on the segment.
        self.start + t * line_direction
    }

    // return the closest point on the segment
    pub fn closest_point_on_segment(&self, point: &Point2<f64>) -> Point2<f64> {
        let line_direction = self.end - self.start;
        let point_direction = *point - self.start;

        let line_length_squared = line_direction.norm_squared();
        if line_length_squared == 0.0 {
            // The line segment has zero length, return the start point.
            return self.start;
        }

        let t = point_direction.dot(&line_direction) / line_length_squared;
        if t < 0.0 {
            // The point is closest to the start of the segment.
            return self.start;
        } else if t > 1.0 {
            // The point is closest to the end of the segment.
            return self.end;
        }

        // The point is closest to a point on the segment.
        self.start + t * line_direction
    }

    // return the closest point on the segment if it falls on him
    pub fn orthogonal_projection_point_on_segment(
        &self,
        point: &Point2<f64>,
    ) -> Option<Point2<f64>> {
        let line_direction = self.end - self.start;
        let point_direction = *point - self.start;

        let line_length_squared = line_direction.norm_squared();
        if line_length_squared == 0.0 {
            // The line segment has zero length, return the start point.
            return None;
        }

        let t = point_direction.dot(&line_direction) / line_length_squared;
        if t < 0. || t > 1. {
            return None;
        } // The point don't fall on the segment.

        // The point is closest to a point on the segment.
        Some(self.start + t * line_direction)
    }

    // takes a number between 0 and 1 and return the point along the line situated at the specified dist
    // for example with 0. it will return the starting point, and with 0.5 it will return the center of the line
    pub fn point_allong_line(&self, x: f64) -> Point2<f64> {
        let p_x = self.start.x + x * (self.end.x - self.start.x);
        let p_y = self.start.y + x * (self.end.y - self.start.y);
        return Point2::new(p_x, p_y);
    }

    // return the middle point of the segment
    pub fn middle(&self) -> Point2<f64> {
        return self.point_allong_line(0.5);
    }

    // return the length of the line
    pub fn norm(&self) -> f64 {
        return ((self.start.x - self.end.x) * (self.start.x - self.end.x)
            + (self.start.y - self.end.y) * (self.start.y - self.end.y))
            .sqrt();
    }
}
