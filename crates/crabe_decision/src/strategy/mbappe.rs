use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;
use crabe_framework::data::output::Kick as KickType;
use crate::action::kick::Kick;
use crate::action::move_to_with_kick::MoveToWithKick;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Mbappe {
    /// The id of the robot to move.
    id: u8,
}

impl Mbappe {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for Mbappe {
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
        let should_kick = |world: &World, id: &u8, tools: &ToolData| {
            // if let Some(r) = world.allies_bot.get(id) {
            //     // robot close to ball
            //     if (r.pose.position - world.ball.as_ref().unwrap().position.xy()).norm() < 1.0 {
            //         return Some(KickType::StraightKick {
            //             power: 1.0,
            //         })
            //     }
            // }

            None
        };
        action_wrapper.push(self.id, MoveToWithKick::new(world.ball.as_ref().unwrap().position.xy(), -PI / 4.0, Box::new(should_kick)));
        action_wrapper.push(self.id, Kick::new(KickType::StraightKick {power: 1.0}));
        false
    }
}
