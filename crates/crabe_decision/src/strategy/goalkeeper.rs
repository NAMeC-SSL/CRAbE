use crabe_math::shape::{Circle, Line, Rectangle};
use std::f64::consts::PI;
use std::ops::Mul;
use std::time::{Duration, Instant};
use crate::action::move_to::{MoveTo, MoveToStar, How};
use crate::action::{Action, ActionWrapper};
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Ball, Robot, TeamColor, World};
use nalgebra::{clamp, Point2, Vector2};
use crabe_framework::data::output::Kick;
use crate::action::order_raw::RawOrder;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
pub struct Goalkeeper {
    /// The id of the robot to move.
    id: u8,
    last_kick: Instant
}

impl Goalkeeper {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, last_kick: Instant::now() }
    }
}

impl Strategy for Goalkeeper {
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

        // let robot_to_ball = ball.position.xy() - robot.pose.position;
        // let robot_to_ball_angle = robot_to_ball.y.atan2(robot_to_ball.x);


        // let mut cmd = MoveTo::new(None, Point2::new(our_goal.x, clamp(ball.position.y, -world.geometry.ally_goal.width/2.0, world.geometry.ally_goal.width/2.0)), robot_to_ball_angle, How::Goal).compute_order(self.id, world, tools_data);
        // if robot_to_ball.norm() < 0.50 && self.last_kick.elapsed() > Duration::from_secs(2) {
        //     println!("GOING TO BALL");
        //     cmd = MoveTo::new(None, ball.position.xy(), robot_to_ball_angle, How::Goal).compute_order(self.id, world, tools_data);
        //     self.last_kick = Instant::now();
        //     cmd.kick = Some(Kick::StraightKick {
        //         power: 1.0
        //     });
        // }

        // // action_wrapper.push(self.id, cmd);
        // action_wrapper.push(self.id, RawOrder::new(cmd));
        // action_wrapper.push(self.id, MoveTo::new(
        //     self.id, None, Point2::new(0.0, 0.0), How::Accurate));
        // panic!("");
        // action_wrapper.push(self.id, MoveToStar::new(
        //     Point2::new(-1.0, 2.0), How::Fast, world.geometry.field.length, world.geometry.field.width));
        // action_wrapper.push(self.id, MoveToStar::new(
        //     self.id, Point2::new(-4.0, 2.0), How::Fast, world.geometry.field.length, world.geometry.field.width));
        // action_wrapper.push(self.id, MoveToStar::new(
        //     self.id, Point2::new(-4.0, -2.0), How::Fast, world.geometry.field.length, world.geometry.field.width));
        // action_wrapper.push(self.id, MoveToStar::new(
        //     self.id, Point2::new(-1.0, -2.0), How::Fast, world.geometry.field.length, world.geometry.field.width));

        
        //calculate the segment between ball and a point his direction
        let ball_dest_point = ball.position + ball.velocity.normalize().mul(100.);
        let ball_dest_point = Point2::new(ball_dest_point.x, ball_dest_point.y);

        let top_left = Point2::new(-world.geometry.field.width / 2., -world.geometry.field.length / 2.);
        let top_right = Point2::new(-world.geometry.field.width / 2., world.geometry.field.length / 2.);


        let ball_line = Line::new(Point2::new(ball.position.x, ball.position.y), ball_dest_point);
        let goal_line = Line::new(top_left, top_right);


        let end_point = ball_line.intersect(goal_line);
        let to_ball = Point2::new(ball.position.x, ball.position.y) - robot.pose.position;
        let a = vector_angle(to_ball);
        let x = world.geometry.ally_goal.top_left_position.x + world.geometry.ally_goal.depth*2.0;
        let y = match end_point {
            None => {
                min(max(ball.position.y, -world.geometry.ally_goal.width/2.0), world.geometry.ally_goal.width/2.0)
            }
            Some(p) => {
                min(max(p.y, -world.geometry.ally_goal.width/2.0), world.geometry.ally_goal.width/2.0)
            }
        };

        action_wrapper.push(self.id, MoveTo::new(None, dbg!(Point2::new(x, y)), a, How::Goal));

        false
    }
}


fn min(a: f64, b:f64) -> f64{
    if a<b {
        return a;}
    b 
}
fn max(a: f64, b:f64) -> f64{
    if a>b {
        return a;}
    b 
}

fn vector_angle(m: Vector2<f64>) -> f64{
    let dir = m.normalize();
    if m.y < 0.0{
        return -dir.x.acos()
    }
    dir.x.acos()
}