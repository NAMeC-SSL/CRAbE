use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The RobotsFormation struct represents a strategy that commands a robot to move in a RobotsFormation shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct RobotsFormation {
    /// The id of the robot to move.
    id: u8,
}

impl RobotsFormation {
    /// Creates a new RobotsFormation instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for RobotsFormation {
    /// Executes the RobotsFormation strategy.
    ///
    /// This strategy commands all the robots to move in the same time to test their movements
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
        for i in 0..=3 {
        action_wrapper.push(0, MoveTo::new(Point2::new(-1.0, 1.5), -PI / 4.0));
        action_wrapper.push(1, MoveTo::new(Point2::new(-1.0, -1.5), -PI / 4.0));
        action_wrapper.push(2, MoveTo::new(Point2::new(-2.0, 1.5), -PI / 4.0));
        action_wrapper.push(3, MoveTo::new(Point2::new(-2.0, -1.5), -PI / 4.0));
        action_wrapper.push(4, MoveTo::new(Point2::new(-3.0, 1.5), -PI / 4.0));
        action_wrapper.push(5, MoveTo::new(Point2::new(-3.0, -1.5), -PI / 4.0));

        action_wrapper.push(0, MoveTo::new(Point2::new(-1.0, 0.5), -PI / 4.0));
        action_wrapper.push(1, MoveTo::new(Point2::new(-1.0, -0.5), -PI / 4.0));
        action_wrapper.push(2, MoveTo::new(Point2::new(-2.0, 0.5), -PI / 4.0));
        action_wrapper.push(3, MoveTo::new(Point2::new(-2.0, -0.5), -PI / 4.0));
        action_wrapper.push(4, MoveTo::new(Point2::new(-3.0, 0.5), -PI / 4.0));
        action_wrapper.push(5, MoveTo::new(Point2::new(-3.0, -0.5), -PI / 4.0));
        }
        true
    }
}
