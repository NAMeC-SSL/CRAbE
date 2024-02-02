use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_framework::data::geometry::Penalty;
use crabe_math::shape::Line;
use nalgebra::Point2;
use std::f64::consts::PI;
use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Defender {
    /// The id of the robot to move.
    id: u8,
}

impl Defender {
    /// Creates a new Defender instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    /// Return a point on the penalty outside line from a number between 0 and 1
    fn line(
        &mut self,
        world: &World,
        x: f64
    ) -> Point2<f64> {
        let enlarged_penalty = world.geometry.ally_penalty.enlarged_penalty(0.3);
        let width = enlarged_penalty.front_line.norm();
        let depth = enlarged_penalty.left_line.norm();    
        let tot_length = depth * 2. + width;
        let dist_along_penalty_line = tot_length * x;
        if dist_along_penalty_line < depth{
            let n_ratio = dist_along_penalty_line/depth;
            return enlarged_penalty.left_line.point_allong_line(n_ratio);
        }else if dist_along_penalty_line < depth + width{
            let n_ratio = (dist_along_penalty_line - depth)/width;
            return enlarged_penalty.front_line.point_allong_line(n_ratio);
        }else{
            let n_ratio = 1. - (dist_along_penalty_line - (depth+width))/depth;
            return enlarged_penalty.right_line.point_allong_line(n_ratio);
        }
    }    

    /// Return the position from 0 to 1 along the penalty zone
    pub fn line_intersection_with_penalty(
        &self, 
        penalty: &Penalty,
        line: Line
    ) ->  Option<f64>{
        let intersect_front_line =  line.intersection_line(&penalty.front_line);
        let penalty_length = penalty.depth *2. + penalty.width;
        if intersect_front_line.is_some(){
            println!("front");
            return Some(((intersect_front_line.unwrap().y - penalty.front_line.start.y).abs() + penalty.depth)/penalty_length);
        }else{
            let intersect_left_line =  line.intersection_line(&penalty.left_line);
            if intersect_left_line.is_some() {
                println!("left");
                return Some(((intersect_left_line.unwrap().x - penalty.left_line.start.x).abs() )/penalty_length);
            }else{
                let intersect_right_line =  line.intersection_line(&penalty.right_line);
                if intersect_right_line.is_some(){
                    println!("right");
                    return Some(((intersect_right_line.unwrap().x - penalty.right_line.end.x).abs() + penalty.depth + penalty.width)/penalty_length);
                }else{
                    println!("ball is in our penalty zone");
                    return None;
                }
            }
        }
    }

    /// Move around the penalty zone
    pub fn oscillate(
        &mut self,
        world: &World,
        action_wrapper: &mut ActionWrapper,
    )-> bool {
        action_wrapper.clear(self.id);
        let current_time = SystemTime::now();
        let mut x = 0.;
        if let Ok(duration) = current_time.duration_since(UNIX_EPOCH) {
            let current_time_ms = duration.as_millis() as f64;
            x = current_time_ms ;
        } 
        let oscillating_value = (0.00005 * 2.0 * std::f64::consts::PI * x).sin() * 0.5 + 0.5;
        let pos = self.line(world, oscillating_value);
        action_wrapper.push(self.id, MoveTo::new(pos, 0.));
        false
    }
}

impl Strategy for Defender {
    fn name(&self) -> &'static str {
        "Defender"
    }

    /// Executes the Defender strategy.
    ///
    /// This strategy commands the robot with the specified ID to move around the goal line
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
        action_wrapper.clear(self.id);

        let ball_pos = match world.ball.clone() {
            None => {return false;}
            Some(ball) => {ball.position.xy() }
        };

        //TODO add this constant in the geometry (see code in rbc branches maybe)
        let goal_center = world.geometry.ally_goal.front_line.middle();
        let ball_to_goal = Line::new(goal_center, ball_pos);

        let intersection_point_ratio = self.line_intersection_with_penalty(&world.geometry.ally_penalty.enlarged_penalty(0.3),ball_to_goal);

        if let Some(ratio) = intersection_point_ratio {
            let pos = self.line(world, ratio);
            action_wrapper.push(self.id, MoveTo::new(pos, 0.));
            println!("Final Intersection Point: {:?}", ratio);
        } else {
            println!("No intersection point found");
        }
        false
    }
}
