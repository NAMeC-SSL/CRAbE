use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::manager::game_manager::GameManager;
use crate::strategy::Strategy;
use crate::constants::{KEEPER_ID, PIVOT_ID, DEFENDER1_ID, DEFENDER2_ID, ATTACKER1_ID, ATTACKER2_ID};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors::{self, vector_from_angle};
use nalgebra::Point2;
use crabe_math::shape::Line;
use std::f64::consts::PI;
use std::ops::{Mul, Add};
use std::time::Instant;

/// The BallPlacement struct represents a strategy that commands the team to set in the BallPlacement formation
/// It is used when the team is not in favor of the freekick
pub struct BallPlacement {
    id: u8,
    point: Point2<f64>,
    state: PlacerState,
    time: Instant
}

impl BallPlacement {
    /// Creates a new BallPlacement instance with the desired robot id.
    /// Add a point in arguments where the robot have to place the ball
    pub fn new(id: u8, point: Point2<f64>) -> Self {
        Self {id, point, state: PlacerState::TakeBall, time: Instant::now()}
    }
}

enum PlacerState{
    TakeBall,
    Place,
    Drop
}

impl Strategy for BallPlacement {
    /// Executes the BallPlacement strategy.
    ///
    /// This strategy commands one robot to place the ball at the point specified
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
        let ball_pos = match world.ball.clone() {
            None => {
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        let robot_pos = robot.pose.position;
        let robot_to_ball = ball_pos - robot_pos;
        let dir_shooting_line: Line = Line::new(robot_pos, robot_pos.add(vector_from_angle(robot.pose.orientation).mul(100.)));
        let dir_shooting_line_ball: Line = Line::new(robot_pos, robot_pos.add((ball_pos - robot_pos).mul(100.)));
        let ball_to_target = self.point - ball_pos;

        let mut behind_ball_pos = ball_pos + ball_to_target.normalize() * -0.3;
        let ball_avoidance: bool = robot_to_ball.normalize().dot(&(self.point-ball_pos).normalize()) < 0.;
        let robot_current_dir = vectors::vector_from_angle(robot.pose.orientation);
        let dot_with_ball = robot_current_dir.normalize().dot(&robot_to_ball.normalize());
        let dot_with_target = robot_current_dir.normalize().dot(&(self.point - robot_pos).normalize());
        match self.state {
            PlacerState::TakeBall => {
                if ((behind_ball_pos - robot_pos).norm() <= 0.2 || (dot_with_ball) > 0.93) && dot_with_target > 0.94{
                    self.state = PlacerState::Place
                }
                if ball_avoidance {
                    let perp_dir=(vectors::rotate_vector((ball_pos - behind_ball_pos), PI/2.)).mul(0.3);
                    let side = -(perp_dir.dot(&robot_to_ball)).signum();
                    behind_ball_pos = behind_ball_pos+perp_dir*side;
                    if GameManager::ball_in_trajectory(world, self.id, behind_ball_pos){
                        behind_ball_pos = robot_pos + perp_dir * side;
                    }
                }
                action_wrapper.push(self.id, MoveTo::new(behind_ball_pos, vectors::angle_to_point(self.point, robot_pos), 0., None, false, false));
            },
            PlacerState::Place => {
                action_wrapper.push(self.id, MoveTo::new(self.point - (self.point - ball_pos).normalize().mul(0.1), vectors::angle_to_point(self.point, robot_pos), 1.0,None, false, false));
                if ball_avoidance || robot_to_ball.norm() > 0.4 || dot_with_target < 0.93{
                    self.state = PlacerState::TakeBall;
                }else if (self.point - ball_pos).norm() <= 0.027{
                    self.state = PlacerState::Drop;
                    self.time = Instant::now();
                }
            }
            PlacerState::Drop => {
                if self.time.elapsed().as_secs() < 1{
                    action_wrapper.push(self.id, MoveTo::new(robot_pos, vectors::angle_to_point(self.point, robot_pos), 0.0,None, false, false));
                }else{
                    action_wrapper.push(self.id, MoveTo::new(self.point - (self.point - ball_pos).normalize().mul(0.2), vectors::angle_to_point(self.point, robot_pos), 0.0,None, false, false));
                    //return true
                }
                if (self.point - ball_pos).norm() > 0.03{
                    self.state = PlacerState::TakeBall;
                }
            },
        };
        false
    }
    fn name(&self) -> &'static str {
        return "BallPlacement";
    }
}

