use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crabe_math::shape::Rectangle;

/// Represents a penalty area on a soccer field.
#[derive(Clone, Debug)]
pub struct Penalty {
    /// The area covered by this penalty zone
    pub area: Rectangle,
}

impl Penalty {
    /// The width of the penalty area in meters, as per the SSL rulebook.
    pub fn width(&self) -> &f64 {
        &self.area.height
    }

    /// The depth of the penalty area in meters, as per the SSL rulebook.
    pub fn depth(&self) -> &f64 {
        &self.area.width
    }
}

impl Serialize for Penalty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let num_fields: usize = 3;
        let mut state =
            serializer.serialize_struct("Penalty", num_fields)?;
        state.serialize_field("width", &self.width())?;
        state.serialize_field("depth", &self.depth())?;
        state.serialize_field("topLeftPosition", &self.area.top_left)?;
        state.end()
    }
}