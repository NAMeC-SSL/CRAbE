use crate::action::move_to::MoveTo;
use crate::action::order_raw::RawOrder;
use crate::action::{ActionWrapper, Actions};
use crate::strategy::Strategy;
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World, self};
use nalgebra::{Point2, Vector2, Point3};
use std::f64::consts::PI;
use std::ops::{Mul, Sub};
#[derive(Default)]
pub struct Goalkeeper {
    id: u8,
}

impl Goalkeeper {
    /// Creates a new Squ_are instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for Goalkeeper {
    /// Executes the Squ_are strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a squ_are shape in a
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
                //println!("{:?}", line_intersect(Point2::new(-5., 0.), Point2::new(3., -4.), Point2::new(-2., -4.), Point2::new(0., 0.)));
                let mut dir=ball.velocity;
                if let Some(robot2) = world.allies_bot.get(&(0)) {
                    dir = ball.position.sub(Point3::new(robot2.pose.position.x, robot2.pose.position.y, 0.));
                }
                //calculate the segment between ball and a point his direction
                let ball_dest_point = ball.position + dir.normalize().mul(100.);
                let ball_dest_point = Point2::new(ball_dest_point.x, ball_dest_point.y);
                println!("{}",ball_dest_point);
                let top_left = Point2::new(-world.geometry.field.width / 2., -world.geometry.field.length / 2.);
                let top_right = Point2::new(-world.geometry.field.width / 2., world.geometry.field.length / 2.);
                let end_point = line_intersect(Point2::new(ball.position.x, ball.position.y), ball_dest_point, top_left, top_right);
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

                action_wrapper.push(self.id, MoveTo::new_dribble(Point2::new(x, y), a));
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


fn line_intersect(a1: Point2<f64>, a2: Point2<f64>, b1: Point2<f64>, b2: Point2<f64>) -> Option<Point2<f64>> {
    let d = (b2.y - b1.y) * (a2.x - a1.x) - (b2.x - b1.x) * (a2.y - a1.y);
    
    if d == 0.{
        return None;
    }
    
    let u_a = ((a1.x - b1.x) * (b1.y - b2.y) - (a1.y - b1.y) * (b1.x - b2.x)) / d;
    let u_b = -((a1.x - a2.x) * (a1.y - b1.y) - (a1.y - a2.y) * (a1.x - b1.x)) / d;
    if !(u_a <= 1. && u_a >= 0. && u_b <= 1. && u_b >= 0.){
        return None;
    }
    
    let x = a1.x + u_a * (a2.x - a1.x);
    let y = a1.y + u_a * (a2.y - a1.y);
    
    Some(Point2::new(x, y))
}