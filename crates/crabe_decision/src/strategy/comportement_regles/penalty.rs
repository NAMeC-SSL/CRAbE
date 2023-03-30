use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Penalty {
    /// The id of the robot to move.
    idOfKicker: u8,
}

impl Penalty {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(idOfKicker: u8) -> Self {
        Self { idOfKicker }
    }
}

impl Strategy for Penalty {
    /// Executes the Square strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a square shape in a
    /// counter-clockwise direction.
    ///
    /// # Arguments
    ///
    /// * world: The current state of the game world.
    /// * tools_data: A collection of external tools used by the strategy, such as a viewer.
    /// * action_wrapper: An `ActionWrapper` instance used to issue actions to the robot.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the strategy is finished or not.
    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        for robot in world.allies_bot.iter() {
            if *robot.0 == self.idOfKicker {
                action_wrapper.push(*robot.0,MoveTo::new(
                    Point2::new(0.0,0.0),PI));
            }
        }
        true
    }
}
