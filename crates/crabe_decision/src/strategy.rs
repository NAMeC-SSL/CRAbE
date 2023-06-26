use crate::action::ActionWrapper;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

/// The `testing` module contains different strategies used for testing purposes. These strategies
/// are not meant to be used in an actual game but rather to test specific functionalities or to
/// experiment with different behaviors.
pub mod testing;
pub mod attacker;

/// The `Strategy` trait defines the interface for a behavior that one or multiple robots can adopt to achieve a certain goal.
/// A strategy receives information about the state of the world and its own state, and issues commands to the robot
/// through an `ActionWrapper` instance. A strategy can run for multiple time steps, until it decides to
/// terminate by returning `true` from the `step` method.
pub trait Strategy {
    /// Executes one step of the strategy, updating the state of the robot and issuing commands
    /// to it through the given `ActionWrapper`.
    ///
    /// # Arguments
    ///
    /// * `world`: The current state of the world.
    /// * `tools_data`: A collection of external tools used by the strategy, such as a viewer.
    /// * `action_wrapper`: An `ActionWrapper` instance used to issue actions to the robot.
    ///
    /// # Returns
    ///
    /// `false` if the strategy should continue running, `true` if it should terminate.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool;
}
