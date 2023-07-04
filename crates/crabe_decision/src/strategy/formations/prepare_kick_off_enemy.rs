use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::constants::{KEEPER_ID, PIVOT_ID, ATTACKER2_ID, DEFENDER2_ID, DEFENDER1_ID, ATTACKER1_ID};
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The PrepareKickOffEnemy struct represents a strategy that commands the team to set in the PrepareKickOffEnemy formation
/// It is used when the team is not in favor of the kick-off
#[derive(Default)]
pub struct PrepareKickOffEnemy {
}

impl PrepareKickOffEnemy {
    /// Creates a new PrepareKickOffEne instance with the desired robot id.
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for PrepareKickOffEnemy {
    /// Executes the PrepareKickOffEnemy strategy.
    ///
    /// This strategy commands all the robots to move in position for kick-off when not in favor
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
        action_wrapper.clean(KEEPER_ID);
        action_wrapper.clean(PIVOT_ID);
        action_wrapper.clean(ATTACKER1_ID);
        action_wrapper.clean(ATTACKER2_ID);
        action_wrapper.clean(DEFENDER1_ID);
        action_wrapper.clean(DEFENDER2_ID);


        let ball_pos = match world.ball.clone() {
            None => {
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        if let Some(robot) = world.allies_bot.get(&DEFENDER2_ID) {
            action_wrapper.push(DEFENDER2_ID, MoveTo::new(Point2::new(-1.2, 0.7), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None , false, false));
        }

        if let Some(robot) = world.allies_bot.get(&PIVOT_ID) {
            action_wrapper.push(PIVOT_ID, MoveTo::new(Point2::new(-1.0, 0.0),vectors::angle_to_point(ball_pos, robot.pose.position),0.0,None, false, false));
        }

        if let Some(robot) = world.allies_bot.get(&DEFENDER1_ID) {
            action_wrapper.push(DEFENDER1_ID, MoveTo::new(Point2::new(-0.7, -1.0), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }

        if let Some(robot) = world.allies_bot.get(&ATTACKER2_ID) {
            action_wrapper.push(ATTACKER2_ID, MoveTo::new(Point2::new(-0.7, 1.0), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }

        if let Some(robot) = world.allies_bot.get(&ATTACKER1_ID) {
            action_wrapper.push(ATTACKER1_ID, MoveTo::new(Point2::new(-1.2, -0.7), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }

        if let Some(robot) = world.allies_bot.get(&KEEPER_ID) {
            action_wrapper.push(KEEPER_ID, MoveTo::new(Point2::new(-4.0, -0.0), vectors::angle_to_point(ball_pos, robot.pose.position), 0.0,None, false, false));
        }
        false
    }
}

