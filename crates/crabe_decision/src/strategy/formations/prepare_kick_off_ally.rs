use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::constants::{KEEPER_ID, PIVOT_ID, DEFENDER1_ID, DEFENDER2_ID, ATTACKER1_ID, ATTACKER2_ID};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The PrepareKickOffAlly struct represents a strategy that commands the team to set in the PrepareKickOffAlly formation
/// It is used when the team is in favor of the kick-off
#[derive(Default)]
pub struct PrepareKickOffAlly {
}

impl PrepareKickOffAlly {
    /// Creates a new PrepareKickOffAlly instance with the desired robot id.
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for PrepareKickOffAlly {
    /// Executes the PrepareKickOffAlly strategy.
    ///
    /// This strategy commands all the robots to move in position for
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
        let ball_pos = match world.ball.clone() {
            None => {
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        if let Some(robot) = world.allies_bot.get(&ATTACKER2_ID) {
            action_wrapper.push(ATTACKER2_ID, MoveTo::new(Point2::new(-0.25, 2.5), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }
        
        if let Some(robot) = world.allies_bot.get(&ATTACKER1_ID) {
            action_wrapper.push(ATTACKER1_ID, MoveTo::new(Point2::new(-0.25, -2.5), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }

        if let Some(robot) = world.allies_bot.get(&PIVOT_ID) {
            action_wrapper.push(PIVOT_ID, MoveTo::new(Point2::new(-0.2, 0.0), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }

        if let Some(robot) = world.allies_bot.get(&DEFENDER1_ID) {
            action_wrapper.push(DEFENDER1_ID, MoveTo::new(Point2::new(-1.5, -1.5), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }

        if let Some(robot) = world.allies_bot.get(&DEFENDER2_ID) {
            action_wrapper.push(DEFENDER2_ID, MoveTo::new(Point2::new(-1.5, 1.5), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }

        if let Some(robot) = world.allies_bot.get(&KEEPER_ID) {
            action_wrapper.push(KEEPER_ID, MoveTo::new(Point2::new(-4.0, -0.0), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }
        true
    }
}

