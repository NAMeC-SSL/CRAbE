use std::f64::consts::PI;
use crabe_framework::data::{world::World, tool::ToolData};
use nalgebra::{ArrayStorage, ComplexField, Const, Matrix, OPoint, Point2};
use crate::{strategy::Strategy, action::{ActionWrapper, move_to::MoveTo}};

const DISTANCE_TO_BALL: f64 = 0.3;
const INACURACY: f64 = 0.01;
const ANGLE_INACURACY: f64 = 0.01;

/// The MoveBall struct represents a strategy that commands a robot to move the ball to a specified position
#[derive(Default)]
pub struct MoveBall {
    /// The id of the robot to move.
    id: u8,
    /// The final position of the ball
    target: OPoint<f64, Const<2>>
}

impl MoveBall {
    /// Creates a new MoveBall instance with the desired robot id and target
    pub fn new(id: u8, target: OPoint<f64, Const<2>>) -> Self {
        Self { id, target }
    }
}

impl Strategy for MoveBall {
    fn name(&self) -> &'static str {
        "MoveBall"
    }

    /// Executes the MoveBall strategy.
    ///
    /// This strategy commands the robot with the specified ID to move the ball to a position
    ///
    /// # Arguments
    ///
    /// * world: The current state of the game world.
    /// * tools_data: A collection of external tools used by the strategy, such as a viewer.    
    /// * action_wrapper: An `ActionWrapper` instance used to issue actions to the robot.
    ///&
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
        let ball = match &world.ball {
            Some(b) => b,
            None => {
                eprintln!("Cannot find ball");
                return false
            }
        }.position_2d();
        
        let robot = &match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                eprintln!("Cannot get robot");
                return false
            }
        }.pose;

        let ball_target = self.target - ball;
        let ball_target_angle = ball_target.y.atan2(ball_target.x);

        let robot_ball = ball - robot.position;
        let robot_ball_angle = robot_ball.y.atan2(robot_ball.x);

        let angle_to_go = if (ball_target_angle - robot_ball_angle).abs() > PI/2.0 {
            if ball_target_angle - robot_ball_angle > 0.0 {
                robot_ball_angle + PI/2.0
            } else {
                robot_ball_angle - PI/2.0
            }
        } else {
            ball_target_angle
        };

        let new_pos: OPoint<f64, Const<2>> = [(angle_to_go - PI).cos()*DISTANCE_TO_BALL + ball.x, (angle_to_go - PI).sin()*DISTANCE_TO_BALL + ball.y].into();

        // let new_pos = (robot.position - ball).normalize()*DISTANCE_TO_BALL;
        // let mut angle = f64::atan(new_pos.y/new_pos.x);
        // if new_pos.x > 0.0 {
        //     angle += PI;
        // }
        // let new_pos = Point2::new(new_pos.x + ball.x, new_pos.y + ball.y);

        action_wrapper.clear(self.id);
        // if (new_pos - robot.position).norm() <= DISTANCE_TO_BALL + INACURACY && angle <= ANGLE_INACURACY {
        //     return true
        // }

        dbg!(new_pos, ball_target_angle);

        action_wrapper.push(self.id, MoveTo::new(new_pos, ball_target_angle));
        false
    }
}
