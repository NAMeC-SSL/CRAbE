use nalgebra::Point2;
use serde::Serialize;

/// Represents a penalty area on a soccer field.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Penalty {
    /// The width of the penalty area in meters.
    pub width: f64,
    /// The depth of the penalty area in meters.
    pub depth: f64,
    /// The top-left corner of the penalty area, measured from the origin of the
    /// field, in meters.
    pub top_left_position: Point2<f64>,
}

impl Penalty {}
