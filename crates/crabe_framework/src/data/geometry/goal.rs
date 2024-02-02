use crabe_math::shape::Line;
use serde::Serialize;

/// Represents a goal on a soccer field. (all distances are in meters)
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    /// The width of the goal
    pub width: f64,
    /// The depth of the goal
    pub depth: f64,
    /// The front line of the goal area
    pub front_line: Line
}

impl Goal {}
