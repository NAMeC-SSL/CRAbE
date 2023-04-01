use crate::action::ActionWrapper;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

pub mod manual;
pub mod game_manager;

/// The `Manager` trait defines a coach that handles the SSL game and gives each robot at least one strategy.
/// A strategy is a behavior for one or multiple robots that gives one `Action` per robot. The `Manager`'s
/// `step` function is called at each game step to update the strategies based on the current game state,
/// and modify the `tools_data` and `action_wrapper` objects accordingly.
pub trait Manager {
    /// The step function is called by the DecisionPipeline on each iteration to compute the
    /// appropriate actions to be taken by the robot based on the current game state.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    );
}
