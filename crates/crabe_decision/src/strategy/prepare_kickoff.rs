use std::f64::consts::PI;
use crate::action::move_to::{MoveTo, MoveToStar, How};
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;

const MIN_DISTANCE_FROM_BALL: f64 = 0.2; // meters

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct PrepareKickoffStrategy {
    /// The id of the robot to move.
    id: u8,
}

impl PrepareKickoffStrategy {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for PrepareKickoffStrategy {
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
        let behind_ball_pos = ball_pos + ball_to_goal.normalize() * -MIN_DISTANCE_FROM_BALL * 1.1;
        // let robot_to_ball_distance = robot_to_ball.norm();

        action_wrapper.push(self.id, MoveTo::new(None, behind_ball_pos, PI, How::Intersept));

        // action_wrapper.push(self.id, MoveTo::new(
        //     self.id, None, Point2::new(0.0, 0.0), How::Accurate));

        // action_wrapper.push(self.id, MoveToStar::new(
        //     Point2::new(-1.0, 2.0), How::Fast, world.geometry.field.length, world.geometry.field.width));
        // action_wrapper.push(self.id, MoveToStar::new(
        //     self.id, Point2::new(-4.0, 2.0), How::Fast, world.geometry.field.length, world.geometry.field.width));
        // action_wrapper.push(self.id, MoveToStar::new(
        //     self.id, Point2::new(-4.0, -2.0), How::Fast, world.geometry.field.length, world.geometry.field.width));
        // action_wrapper.push(self.id, MoveToStar::new(
        //     self.id, Point2::new(-1.0, -2.0), How::Fast, world.geometry.field.length, world.geometry.field.width));
        true
    }
}
