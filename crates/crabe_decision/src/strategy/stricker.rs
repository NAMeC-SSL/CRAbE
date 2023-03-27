use crate::action::move_to::MoveTo;
use crate::action::order_raw::RawOrder;
use crate::action::{ActionWrapper, Actions};
use crate::strategy::Strategy;
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World, self};
use nalgebra::{Point2, Vector2};
use std::f64::consts::PI;
use std::ops::Mul;

#[derive(Default)]
pub struct Stricker {
    id: u8,
}

impl Stricker {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for Stricker {
    /// Executes the Square strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a square shape in a
    /// counter-clockwise direction.
    ///
    /// # Arguments
    ///
    /// * data: The current state of the game world.
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
        if let Some(ball) = world.ball.as_ref() {
            //action_wrapper.push(self.id, MoveTo::new(Point2::new(ball.position.x, ball.position.y), 0.0));
            if let Some(robot) = world.allies_bot.get(&(self.id)) {
                let to_ball = robot.pose.position - Point2::new(ball.position.x, ball.position.y);
                if robot.has_ball || to_ball.magnitude() <= 0.11 {
                    println!("kick");
                    let cmd = Command {
                        forward_velocity: 0.0,
                        left_velocity: 0.0,
                        angular_velocity: 0.0,
                        charge: true,
                        kick: Some(Kick::StraightKick{power:4.0}),
                        dribbler: 1.,
                    };
                    action_wrapper.push(self.id, RawOrder::new(cmd));
                }
                let mut dir = Point2::new(ball.position.x, ball.position.y) - world.geometry.ally_goal.back_center_position();
                dir = dir.normalize().mul(0.09);
                let to_goal = world.geometry.ally_goal.back_center_position() - robot.pose.position;
                let a = vector_angle(to_goal);
                action_wrapper.push(self.id, MoveTo::new(Point2::new(ball.position.x + dir.x, ball.position.y + dir.y), a));
            
            }
        }
        false
    }
}

fn vector_angle(m: Vector2<f64>) -> f64{
    let dir = m.normalize();
    if m.y < 0.0{
        return -dir.x.acos()
    }
    dir.x.acos()
}