use crate::data::world::{Team, TeamColor};
use serde::Serialize;
use crate::data::world::game_state::{GameState, HaltedState};

/// The `GameData` struct represents the state of the SSL game, including the teams and which team is on the positive half of the field.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    /// The `Team` struct representing our ally team.
    pub ally: Team,
    /// The `Team` struct representing the enemy team.
    pub enemy: Team,
    /// The color of the team that is on the positive half of the field.
    pub positive_half: TeamColor,
    /// The current game state
    pub state: GameState
}

impl GameData {
    /// Creates a new `GameData` with the given `team_color` as the team color for the ally team, and the opposite team color for the enemy team.
    pub fn new(team_color: TeamColor) -> Self {
        Self {
            ally: Team::with_color(team_color),
            enemy: Team::with_color(team_color.opposite()),
            positive_half: team_color.opposite(),
            state: GameState::Halted(HaltedState::Halt),
        }
    }
}
 