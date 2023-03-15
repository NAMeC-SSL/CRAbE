use nalgebra::Point2;
use serde::Serialize;

/// Represents a goal on a soccer field.
#[derive(Serialize, Clone, Debug)]
pub struct Goal {
    /// The width of the goal, in meters.
    pub width: f64,
    /// The depth of the goal, in meters.
    pub depth: f64,
    /// The top-left corner of the goal, measured from the origin of the field,
    /// in meters.
    pub top_left_position: Point2<f64>,
}

impl Goal {
    pub fn top_center_position(&self) -> Point2<f64>{
        let goal_x = self.top_left_position.x- self.depth/2.;
        let goal_y = self.top_left_position.y - self.width/2.;
        Point2::new(goal_x, goal_y)
    }
    pub fn bottom_left_position(&self) -> Point2<f64>{
        let goal_x = self.top_left_position.x- self.depth;
        let goal_y = self.top_left_position.y;
        Point2::new(goal_x, goal_y)
    }
    pub fn bottom_right_position(&self) -> Point2<f64>{
        let goal_x = self.top_left_position.x- self.depth;
        let goal_y = self.top_left_position.y - self.width;
        Point2::new(goal_x, goal_y)
    }
}
