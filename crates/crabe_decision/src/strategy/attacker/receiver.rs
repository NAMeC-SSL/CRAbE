use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::constants::{PIVOT_ID, ATTACKER1_ID, ATTACKER2_ID};
use crate::manager::game_manager::GameManager;
use crate::strategy::Strategy;
use crabe_framework::data::output::Kick;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::{Point2};
use std::ops::{Add, Mul};
use crabe_math::vectors::{self, vector_from_angle};
use crabe_math::shape::Line;

#[derive(Default)]
pub struct Receiver {
    /// The id of the robot to move.
    id: u8,
    sender_id: u8,
}
impl Receiver {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8, sender_id: u8) -> Self {
        Self { id, sender_id}
    }
}

impl Strategy for Receiver {
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
        action_wrapper.clean(self.id);
        let robot = match world.allies_bot.get(&self.id) {
            None => {
                return false;
            }
            Some(robot) => {
                robot
            }
        };
        let robot_sender = match world.allies_bot.get(&self.sender_id) {
            None => {
                return false;
            }
            Some(robot) => {
                robot
            }
        };
        let ball = match world.ball.clone() {
            None => {
                return false;
            }
            Some(ball) => {
                ball
            }
        };
        let ball_pos = ball.position_2d();
        let dir = (ball_pos-robot_sender.pose.position).normalize();
        let mut interseption_line = Line::new(ball_pos + dir,ball_pos + dir.mul(100.));
        if ball.velocity.norm() > 0.{
            let ball_dir = ball.position + (ball.velocity * 1000.);
            interseption_line.end = ball_dir.xy();
        }

        let interseption_point = interseption_line.get_closest_point(&robot.pose.position);
        tools_data.annotations.add_line("receiver interseption line".to_owned(), interseption_line);

        let dribbler = if (robot.pose.position - ball_pos).norm() < 0.3 { 1.} else{0.};
        action_wrapper.push(self.id, MoveTo::new(interseption_point, vectors::angle_to_point(ball_pos, robot.pose.position), dribbler, None, false, true));
        false
    }

    fn name(&self) -> &'static str{
        return "Receiver";
    }
}
