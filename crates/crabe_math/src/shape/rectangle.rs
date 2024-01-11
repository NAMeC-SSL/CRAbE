use nalgebra::{Point2};
use serde::Serialize;

/// A rectangle in 2D space, defined by a width, a height, and a position.
/// 
/// Note that the `width` and `height` fields should have the same units of
/// measurement as the coordinates of the `position` field.
///
/// The base (in French: repère) is defined as follows :
/// ```txt
/// * -- -- -- >
/// |          x
/// |
/// |
/// v y
/// ```
#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rectangle {
    /// The width of the rectangle.
    pub width: f64,
    /// The height of the rectangle.
    pub height: f64,
    /// The position of the rectangle's top-left corner.
    pub top_left: Point2<f64>,
    /// The position of the rectangle's center.
    pub center: Point2<f64>,
}

impl Rectangle {
    /// Creates a new Rectangle by using the top-left Point2 as the reference
    pub fn new(width: f64, height: f64, top_left: Point2<f64>) -> Rectangle {
        Rectangle {
            width,
            height,
            top_left,
            center: Point2::new(top_left.x + (width / 2.), top_left.y - (height / 2.)),
        }
    }

    pub fn top_right(&self) -> Point2<f64> {
        Point2::new(self.top_left.x + self.width, self.top_left.y)
    }

    pub fn bottom_right(&self) -> Point2<f64> {
        Point2::new(self.top_left.x + self.width, self.top_left.y - self.height)
    }

    pub fn bottom_left(&self) -> Point2<f64> {
        Point2::new(self.top_left.x, self.top_left.y - self.height)
    }
}