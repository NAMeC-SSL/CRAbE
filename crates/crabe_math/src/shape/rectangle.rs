use nalgebra::Point2;
use serde::Serialize;

/// A rectangle in 2D space, defined by a width, a height, and a position.
///
/// Note that the `width` and `height` fields should have the same units of
/// measurement as the coordinates of the `position` field.
#[derive(Clone, Serialize, Debug)]
pub struct Rectangle {
    /// The width of the rectangle.
    pub width: f64,
    /// The height of the rectangle.
    pub height: f64,
    /// The position of the rectangle's top-left corner.
    pub position: Point2<f64>,
}
