use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;
use crate::action::move_to_avoid::MoveToAvoid;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Square {
    /// The id of the robot to move.
    id: u8,
}

impl Square {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for Square {
    fn name(&self) -> &'static str {
        "Square"
    }

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
        for i in 0..3 {
            action_wrapper.push(self.id, MoveToAvoid::new(Point2::new(-1.0, 1.0), 0f64, (*world).clone()));
            action_wrapper.push(self.id, MoveTo::new(Point2::new(1.0, 1.0), 0f64));
            action_wrapper.push(self.id, MoveTo::new(Point2::new(1.0, -1.0), 0f64));
            action_wrapper.push(self.id, MoveTo::new(Point2::new(-1.0, -1.0), 0f64));
        }
        true
    }
}
