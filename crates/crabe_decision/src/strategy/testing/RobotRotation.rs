use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The Rotate struct represents a strategy that commands a robot to Rotate
/// It is used for testing purposes.
#[derive(Default)]
pub struct Rotate {
    /// List of the robots to move.
    ids: Vec<u8>,
}

impl Rotate {
    /// Creates a new Rotate instance with the desired robot id.
    pub fn new(ids: Vec<u8>) -> Self {
        Self { ids }
    }
}

impl Strategy for Rotate {
    fn name(&self) -> &'static str {
        "Rotate"
    }

    /// Executes the Rotate strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a Rotate shape in a
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
        for id in &self.ids {
            ActionWrapper::clear(action_wrapper, *id);
            println!("Robot {} is rotating", id);
            let robot_pos = match world.allies_bot.get(id) {
                None => {
                    return false;
                }
                Some(robot) => {
                    robot.pose.position
                }
            };

            let robot_orientation = match world.allies_bot.get(id) {
                None => {
                    return false;
                }
                Some(robot) => {
                    robot.pose.orientation
                }
            };

            action_wrapper.push(*id, MoveTo::new(robot_pos, robot_orientation + PI/ 2.0));
        }
        // false means that the strategy is not finished and it will repeat
        false
    }
}
