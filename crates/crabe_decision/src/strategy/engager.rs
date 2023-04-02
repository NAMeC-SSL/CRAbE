use std::f64::consts::PI;
use std::time::{Duration, Instant};
use crate::action::move_to::{MoveTo, MoveToStar, How};
use crate::action::{Action, ActionWrapper};
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Ball, Robot, TeamColor, World};
use nalgebra::{clamp, Point2};
use crabe_framework::data::output::Kick;
use crate::action::order_raw::RawOrder;


/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
pub struct Engager {
    /// The id of the robot to move.
    id: u8,
    last_kick: Instant
}

impl Engager {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, last_kick: Instant::now() }
    }
}

impl Strategy for Engager {
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
        let our_goal: Point2<f64> = Point2::new(-4.3, 0.0);

        action_wrapper.clean(self.id);

        let robot = match world.allies_bot.get(&self.id) {
            None => {
                return false;
            }
            Some(r) => r
        };

        let ball = match &world.ball {
            None => {
                return false;
            }
            Some(b) => b
        };
        
        //let cmd = MoveTo::new(None, Point2::new(-1, 0), 0., How::Kick);

        // action_wrapper.push(self.id, cmd);
        //action_wrapper.push(self.id, RawOrder::new(cmd));
        false
    }
}
