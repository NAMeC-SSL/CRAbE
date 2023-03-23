use serde::Serialize;
use crate::data::annotation::AnnotationStore;

/// The `ToolData` struct is a container for storing additional data that can be sent to
/// external tools, such as a viewer or joystick handler.
#[derive(Clone, Default, Serialize)]
pub struct ToolData {
    #[serde(flatten)]
    pub annotations: AnnotationStore
}

/// The `ToolCommands` struct is a container for storing commands that are sent to external
/// tools.
pub struct ToolCommands;
