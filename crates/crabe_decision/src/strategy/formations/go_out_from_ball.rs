use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::constants::{KEEPER_ID, PIVOT_ID, DEFENDER1_ID, DEFENDER2_ID, ATTACKER1_ID, ATTACKER2_ID};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors;
use nalgebra::Point2;
use crabe_math::shape::Line;
use std::f64::consts::PI;
use std::ops::Mul;

/// The PrepareFreeKickEnemy struct represents a strategy that commands the team to set in the PrepareFreeKickEnemy formation
/// It is used when the team is not in favor of the freekick
#[derive(Default)]
pub struct GoOutFromBall {
}

impl GoOutFromBall {
    /// Creates a new PrepareFreeKickEnemy instance with the desired robot id.
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for GoOutFromBall {
    /// Executes the PrepareFreeKickEnemy strategy.
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
                Point2::new(0.0,0.0)
            }
            Some(ball) => {
                ball.position.xy()
            }
        };    
        for i in 0..6{
            if let Some(bappe) = world.allies_bot.get(&i) {
                let to_ball=ball_pos - bappe.pose.position;
                if to_ball.norm() < 0.6 {
                    action_wrapper.clean(bappe.id);
                    action_wrapper.push(bappe.id, MoveTo::new(bappe.pose.position - to_ball.normalize().mul(0.6), vectors::angle_to_point(ball_pos, bappe.pose.position), 0.0,None, false, false));
                }
            };
        }
        false
    }
    fn name(&self) -> &'static str {
        return "PrepareFreeKickEnemy";
    }
}

