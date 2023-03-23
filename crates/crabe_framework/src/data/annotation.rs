use std::collections::HashMap;
use nalgebra::Point2;
use serde::{Deserialize, Serialize};
use crabe_math::shape::{Circle, Line, Rectangle};
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
/// A structure for storing and managing named annotations to be drawn on the SSL RoboCup field viewer.
pub struct AnnotationStore {
    #[serde_as(as = "Vec<(_, _)>")]
    annotations: HashMap<String, Annotation>
}
/// An enumeration representing various annotation types that can be displayed in the SSL RoboCup field viewer.
#[derive(Clone, Serialize)]
#[serde(tag = "kind", content = "content", rename_all = "camelCase")]
pub enum Annotation {
    Circle(Circle),
    Line(Line),
    Rectangle(Rectangle),
    Point(Point2<f64>),
}

impl AnnotationStore {
    /// Add a circle annotation to be displayed in the field viewer.
    ///
    /// # Arguments
    ///
    /// * `id`: A unique identifier for the annotation.
    /// * `circle`: The circle shape to be added as an annotation.
    pub fn add_circle(&mut self, id: String, circle: Circle) {
        self.annotations.insert(id, Annotation::Circle(circle));
    }

    /// Add a point annotation to be displayed in the field viewer.
    ///
    /// # Arguments
    ///
    /// * `id`: A unique identifier for the annotation.
    /// * `point`: The 2D point to be added as an annotation.
    pub fn add_point(&mut self, id: String, point: Point2<f64>) {
        self.annotations.insert(id, Annotation::Point(point));
    }

    /// Add a rectangle annotation to be displayed in the field viewer.
    ///
    /// # Arguments
    ///
    /// * `id`: A unique identifier for the annotation.
    /// * `rectangle`: The rectangle shape to be added as an annotation.
    pub fn add_rectangle(&mut self, id: String, rectangle: Rectangle) {
        self.annotations.insert(id, Annotation::Rectangle(rectangle));
    }

    /// Remove all annotations from the store.
    pub fn clear(&mut self) {
        self.annotations.clear();
    }

    /// Remove an annotation from the store by its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `id`: The unique identifier of the annotation to be removed.
    pub fn remove(&mut self, id: &str) {
        self.annotations.remove(id);
    }

    /// Retrieve a reference to an annotation by its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `id`: The unique identifier for the annotation.
    ///
    /// # Returns
    ///
    /// An `Option<&Annotation>` containing a reference to the annotation if it exists, or `None` if not found.
    pub fn get(&self, id: &str) -> Option<&Annotation> {
        self.annotations.get(id)
    }
}