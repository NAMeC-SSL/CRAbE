use crate::action::move_to::{How, MoveTo};
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Ball, Robot, World};
use nalgebra::Point2;
use std::f64::consts::PI;
use std::ops::{Sub, Mul, Add};
use crabe_framework::data::output::{Command, Kick as KickType};
use crate::action::kick::Kick;
use crate::action::order_raw::RawOrder;
use crabe_math::shape::Line;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default, Debug)]
pub struct Mbappe {
    /// The id of the robot to move.
    id: u8,
    internal_state: MbappeState
}

#[derive(Debug, Default)]
enum MbappeState {
    #[default]
    GoingBehindBall,
    GoingCloseBehindBall
}

impl Mbappe {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, internal_state: MbappeState::GoingBehindBall }
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
        action_wrapper.clean(self.id);
        //
        // let should_kick = |world: &World, id: &u8, tools: &ToolData| {
        //     // if let Some(r) = world.allies_bot.get(id) {
        //     //     // robot close to ball
        //     //     if (r.pose.position - world.ball.as_ref().unwrap().position.xy()).norm() < 1.0 {
        //     //         return Some(KickType::StraightKick {
        //     //             power: 1.0,
        //     //         })
        //     //     }
        //     // }
        //
        //     None
        // };
        let goal_pos: Point2<f64> = Point2::new(4.5, 0.0);
        let ball_pos = match world.ball.clone() {
            None => {
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        let robot_pos = match world.allies_bot.get(&self.id) {
            None => {
                return false;
            }
            Some(robot) => {
                robot.pose.position
            }
        };

        let robot_to_ball = ball_pos - robot_pos;
        let robot_to_ball_angle = robot_to_ball.y.atan2(robot_to_ball.y);
        let robot_to_goal = goal_pos - robot_pos;
        let robot_to_goal_angle = robot_to_goal.y.atan2(robot_to_goal.x);
        let ball_to_goal = goal_pos - ball_pos;
        let behind_ball_pos = ball_pos + ball_to_goal.normalize() * -0.4;
        let close_after_ball_pos = ball_pos + ball_to_goal.normalize() * 0.05;

        let robot_to_ball_distance = robot_to_ball.norm();

        dbg!(&self.internal_state);
        match dbg!(&self.internal_state) {
            MbappeState::GoingBehindBall => {
                if (behind_ball_pos - robot_pos).norm() < 0.1 {
                    self.internal_state = MbappeState::GoingCloseBehindBall;
                } else {
                    action_wrapper.push(self.id, MoveTo::new(None, behind_ball_pos, robot_to_ball_angle, How::Fast));
                }
            }
            MbappeState::GoingCloseBehindBall => {
                let ball_dir = Point2::new(ball_pos.x, ball_pos.y).sub(robot_pos);
                let ball_dest_point = Point2::new(ball_pos.x, ball_pos.y) + ball_dir.normalize().mul(100.);
                let ball_dest_point = Point2::new(ball_dest_point.x, ball_dest_point.y);
        
                let top_left = world.geometry.enemy_goal.top_left_position;
                let mut top_right = world.geometry.enemy_goal.top_left_position;
                top_right.x = -top_right.x;
        
                let ball_line = Line::new(Point2::new(ball_pos.x, ball_pos.y), ball_dest_point);
                let goal_line = Line::new(top_left, top_right);
        
                let aiming_goal = match ball_line.intersect(goal_line){
                    None => false,
                    Some(p) => true
                };
                if dbg!(robot_to_ball_distance) < 0.11  && dbg!(robot_to_ball_angle.abs()) < PI/3.0 { // && aiming_goal{
                    println!("KICK");
                    action_wrapper.push(self.id, Kick::new(KickType::StraightKick {power: 1.0}));
                    self.internal_state = MbappeState::GoingBehindBall;
                } else {
                    action_wrapper.push(self.id, MoveTo::new_dribbling(None, dbg!(close_after_ball_pos), robot_to_goal_angle, How::Accurate));
                }
            }
        }


        // action_wrapper.push(self.id, MoveToWithKick::new(world.ball.as_ref().unwrap().position.xy(), -PI / 4.0, Box::new(should_kick)));
        // action_wrapper.push(self.id, Kick::new(KickType::StraightKick {power: 10.0}));

        false
    }
}
