use log::info;
use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::GameState;
use crabe_framework::data::world::World;
use crate::strategy::testing::Square;

/// The `Manual` struct represents a decision manager that executes strategies manually
/// added to its list.
/// It's used for testing individual strategies only and not meant to be used during an actual game.
///
/// To add a strategy, simply create a new instance of the desired strategy and add it to the
/// `strategies` field in the `new()` method of the `Manual` struct.
#[derive(Default)]
pub struct Karen { // Karen says what to do lmao
    last_game_state: Option<GameState>,
    strategy: Option<Box<dyn Strategy>>,
}

impl Karen {
    /// Creates a new `Manual` instance with the desired strategies to test.
    pub fn new() -> Self {
        Self {
            last_game_state: None,
            strategy: None
        }
    }
}

impl Manager for Karen {
    /// Executes the list of strategies on the given `World` data, `ToolData`, and `ActionWrapper`.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) {
        if self.last_game_state.is_none() || self.last_game_state.unwrap() != world.data.state {
            info!("clearing strategy");
            // clear current strategy
            self.strategy = None;
            for id in world.allies_bot.keys() {
                action_wrapper.clean(*id);
            }

            match world.data.state {
                GameState::Halted(_) => {}
                GameState::Stopped(_) => {}
                GameState::Running(_) => {
                    info!("setting strategy to square");
                    self.strategy = Some(Box::new(Square::new(0)));
                }
            }
        }


        if let Some(strategy) = &mut self.strategy {
            strategy.step(world, tools_data, action_wrapper);
        }

        self.last_game_state = Some(world.data.state);
    }
}
