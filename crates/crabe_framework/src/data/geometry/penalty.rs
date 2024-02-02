use serde::Serialize;
use crabe_math::shape::Line;

/// Represents a penalty area on a soccer field. (all distances are in meters)
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Penalty {
    /// The width of the penalty area 
    pub width: f64,
    /// The depth of the penalty area 
    pub depth: f64,
    /// The front line of the penalty area
    pub front_line: Line,
    /// The left line of the penalty area (looking from the center of the field)
    pub left_line: Line,
    /// The right line of the penalty area (looking from the center of the field)
    pub right_line: Line,
}

impl Penalty {}
