use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::testing::{GoToCenter, Square};
use crate::strategy::formations::BallPlacement;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use crate::strategy::attacker::Shooter;
use crate::strategy::keeper::Keep;

/// The `Manual` struct represents a decision manager that executes strategies manually
/// added to its list.
/// It's used for testing individual strategies only and not meant to be used during an actual game.
///
/// To add a strategy, simply create a new instance of the desired strategy and add it to the
/// `strategies` field in the `new()` method of the `Manual` struct.
#[derive(Default)]
pub struct Manual {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Manual {
    /// Creates a new `Manual` instance with the desired strategies to test.
    pub fn new() -> Self {
        Self {
            strategies: vec![Box::new(BallPlacement::new(0, Point2::new(0.,0.)))],
        }
    }
}

impl Manager for Manual {
    /// Executes the list of strategies on the given `World` data, `ToolData`, and `ActionWrapper`.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) {
        self.strategies
            .retain_mut(|s| !s.step(world, tools_data, action_wrapper));
    }
}
