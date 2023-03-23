use serde::{Deserialize, Serialize};

/// The `TeamColor` enum represents the color of a team in the SSL game, either blue or yellow.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TeamColor {
    Blue,
    Yellow,
}

impl TeamColor {
    /// Returns the opposite color of the current color.
    pub fn opposite(&self) -> Self {
        match self {
            TeamColor::Blue => TeamColor::Yellow,
            TeamColor::Yellow => TeamColor::Blue,
        }
    }
}

/// The `Team` struct represents a team in the SSL game, including its color and optional name.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    /// The color of the team.
    color: TeamColor,
    /// The name of the team, if provided.
    name: Option<String>,
}

impl Team {
    /// Creates a new team with the given color.
    pub fn with_color(color: TeamColor) -> Self {
        Self { color, name: None }
    }
}
