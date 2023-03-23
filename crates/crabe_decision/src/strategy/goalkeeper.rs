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
pub struct Goalkeeper {
    id: u8,
}

impl Goalkeeper {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
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
                
                let ball_dest_point = ball.position + ball.velocity.normalize().mul(100.);
                let ball_dest_point = Point2::new(ball_dest_point.x, ball_dest_point.y);
                let end_point = line_intersect(Point2::new(ball.position.x, ball.position.y), ball_dest_point, world.geometry.enemy_goal.front_left_position(), world.geometry.enemy_goal.front_right_position());
                let to_ball = Point2::new(ball.position.x, ball.position.y) - robot.pose.position;
                let a = vector_angle(to_ball);
                let x = world.geometry.ally_goal.top_left_position.x + world.geometry.ally_goal.depth*2.0;
                let mut y = ball.position.y;
                y = min(y, world.geometry.ally_goal.width/2.0);
                y = max(y, -world.geometry.ally_goal.width/2.0);
                y = match end_point {
                    None => {
                        return false;
                    }
                    Some(p) => {
                        p.y
                    }
                };

                action_wrapper.push(self.id, MoveTo::new(dbg!(Point2::new(x, y)), a));
            }
        }
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


fn line_intersect(A1: Point2<f64>, A2: Point2<f64>, B1: Point2<f64>, B2: Point2<f64>) -> Option<Point2<f64>> {
    let d = (B2.y - B1.y) * (A2.x - A1.x) - (B2.x - B1.x) * (A2.y - A1.y);
    
    if d == 0.{
        return None;
    }
    
    let uA = ((B2.x - B1.x) * (A1.y - B1.y) - (B2.y - B1.y) * (A1.x - B1.x)) / d;
    let uB = ((A2.x - A1.x) * (A1.y - B1.y) - (A2.y - A1.y) * (A1.x - B1.x)) / d;

    if !(uA <= 1. && uA >= 0. && uB <= 1. && uB >= 0.){
        return None;
    }
    
    let x = A1.x + uA * (A2.x - A1.x);
    let y = A1.y + uA * (A2.y - A1.y);
    
    Some(Point2::new(x, y))
}