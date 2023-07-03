use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::manager::game_manager::GameManager;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World, Robot, EnemyInfo, AllyInfo};
use crabe_math::shape::Line;
use crabe_math::vectors::{vector_from_angle, self};
use nalgebra::{Point2, clamp};
use std::cmp::min;
use std::f64::consts::PI;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Keep {
    /// The id of the robot to move.
    id: u8,
}

impl Keep {
    /// Creates a new Keep instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for Keep {
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
        action_wrapper.clean(self.id);       
        let robot = match world.allies_bot.get(&self.id) {
            None => {
                return false;
            }
            Some(robot) => {
                robot
            }
        };
        if let Some(ball) = &world.ball{
            let mut shoot_dir = Line::new(ball.position_2d(),Point2::new(-10.,ball.position.y));
            if ball.velocity.norm() > 0.{//TODO : ball velocity is equal to 0
                let ball_dir = ball.position + ball.velocity * 1000.;
                shoot_dir.end = ball_dir.xy();
            }
            else if let Some(closest_enemy) = GameManager::closest_enemy_to_ball(world){
                let enemy_dir = closest_enemy.pose.position + vector_from_angle(closest_enemy.pose.orientation) * 1000.;
                shoot_dir.end = enemy_dir.xy();
            }
            if let Some(intersection) = world.geometry.ally_goal.front_line.intersection_line(&shoot_dir) {
                let x = world.geometry.ally_goal.bottom_left_position.x+0.1;
                dbg!(x);
                let y = clamp(intersection.y, world.geometry.ally_goal.bottom_left_position.y, world.geometry.ally_goal.bottom_right_position.y);
                action_wrapper.push(self.id, MoveTo::new(Point2::new(x, y), vectors::angle_to_point(ball.position.xy(), robot.pose.position ), 0., None, false, false));
            }
        }
        false
    }
}
