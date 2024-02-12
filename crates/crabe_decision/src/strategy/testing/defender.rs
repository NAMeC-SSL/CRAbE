use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_framework::data::geometry::Penalty;
use crabe_math::shape::Line;
use nalgebra::Point2;
use std::time::{SystemTime, UNIX_EPOCH};

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Defender {
    /// The id of the robot to move.
    ids: Vec<u8>,
    current_pos_along_penaly: f64
}

impl Defender {
    /// Creates a new Defender instance with the desired robot id.
    pub fn new(ids: Vec<u8>) -> Self {
        Self { ids, current_pos_along_penaly: 0.5 }
    }

    /// Return a point on the penalty outside line from a number between 0 and 1
    fn on_penalty_line(
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
        line: Line,
        strict_segment_intersection: bool//true : check intersection with segment line; false : check intersection with infinite line
    ) ->  Option<f64>{
        let intersect_front_line = if strict_segment_intersection {line.intersection_segment(&penalty.front_line)} else {penalty.front_line.intersection_segment_line(&line)};
        let penalty_length = penalty.depth *2. + penalty.width;
        if intersect_front_line.is_some(){
            //intersect front line
            return Some(((intersect_front_line.unwrap().y - penalty.front_line.start.y).abs() + penalty.depth)/penalty_length);
        }else{
            let intersect_left_line = if strict_segment_intersection {line.intersection_segment(&penalty.left_line)} else {penalty.left_line.intersection_line(&line)};
            if intersect_left_line.is_some() {
                //intersect left line
                return Some(((intersect_left_line.unwrap().x - penalty.left_line.start.x).abs() )/penalty_length);
            }else{
                let intersect_right_line = if strict_segment_intersection {line.intersection_segment(&penalty.right_line)} else {penalty.right_line.intersection_line(&line)};
                if intersect_right_line.is_some(){
                    //intersect right line
                    return Some(((intersect_right_line.unwrap().x - penalty.right_line.end.x).abs() + penalty.depth + penalty.width)/penalty_length);
                }else{
                    if strict_segment_intersection {println!("ball may be in our penalty zone");}
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
        for id in self.ids.clone() {
            action_wrapper.clear(id);
        }
        let current_time = SystemTime::now();
        let mut x = 0.;
        if let Ok(duration) = current_time.duration_since(UNIX_EPOCH) {
            let current_time_ms = duration.as_millis() as f64;
            x = current_time_ms ;
        } 
        let oscillating_value = (0.00005 * 2.0 * std::f64::consts::PI * x).sin() * 0.5 + 0.5;
        let pos = self.on_penalty_line(world, oscillating_value);
        for id in self.ids.clone() {
            action_wrapper.push(id, MoveTo::new(pos, 0.));
        }
        false
    }

    pub fn get_closest_point_on_penalty_line(
        &mut self,
        world: &World,
        action_wrapper: &mut ActionWrapper,
    ) -> Option<f64> {  
        //THIS FUNCTION DO NOT WORK PROPERLY
        //TODO refactor to prevent code redundance
        let goal_center = world.geometry.ally_goal.front_line.middle();
        let mut total = 0.;
        let mut total_bot_nb = 0.;//in case some bots don't have intersection line we shouldn't count them in the mean
        for id in &self.ids{
            if world.allies_bot.len() as u8 >= *id{
                let bot_pos = world.allies_bot[id].pose.position;
                let bot_to_goal = Line::new(goal_center, bot_pos);
                if let Some(bot_ratio_pos) = self.line_intersection_with_penalty(&world.geometry.ally_penalty.enlarged_penalty(0.3), bot_to_goal, false) {
                    total += bot_ratio_pos;
                    total_bot_nb += 1.;
                }
            }
        }
        if total_bot_nb <= 0. {
            return None;
        }
        Some(total/total_bot_nb)
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
        for id in &self.ids{
            action_wrapper.clear(*id);
        }
        
        let ball_pos = match world.ball.clone() {
            None => {return false;}
            Some(ball) => {ball.position.xy() }
        };

        let goal_center = world.geometry.ally_goal.front_line.middle();
        let ball_to_goal = Line::new(goal_center, ball_pos);

        let intersection_point_ratio = self.line_intersection_with_penalty(&world.geometry.ally_penalty.enlarged_penalty(0.3),ball_to_goal, true);

        if let Some(mut ratio) = intersection_point_ratio {//if ball to goal center intersect the penalty line
            if let Some(current_pos) = self.get_closest_point_on_penalty_line(world, action_wrapper){
                self.current_pos_along_penaly = current_pos;
            }
            //clamp new bot position so they have to move along the penalty line
            //REMOVE COMMENT 
            //ratio = ratio.clamp(self.current_pos_along_penaly-0.1, self.current_pos_along_penaly+0.1);

            println!("{:?}", self.current_pos_along_penaly);
			//TODO refactor this code (redundance in the on_penalty_line)
            let enlarged_penalty = world.geometry.ally_penalty.enlarged_penalty(0.3);
            let width = enlarged_penalty.front_line.norm();
            let depth = enlarged_penalty.left_line.norm();    
            let tot_penalty_line_length = depth * 2. + width;

			//TODO replace 0.2 with reel bot diameter constant
            let bot_diameter = 0.2;
			let bot_nb = self.ids.len() as f64;
			let bot_diameter_to_ratio = bot_diameter / tot_penalty_line_length; // bot diameter between 0 and 1
			let starting_pos = (ratio - (bot_diameter_to_ratio/2.)*(bot_nb-1.)).clamp(0., 1.-(bot_nb-1.)*bot_diameter_to_ratio);

            let mut i = 0;
            for id in self.ids.clone() {
                let relative_ratio = starting_pos + (i as f64) * bot_diameter_to_ratio;
                let pos = self.on_penalty_line(world, relative_ratio);
                action_wrapper.push(id, MoveTo::new(pos, 0.));
                i+=1;
            }
            // action_wrapper.clear(4);
            // action_wrapper.push(4, MoveTo::new(self.on_penalty_line(world, self.current_pos_along_penaly), 0.));
            println!("Final Intersection Point: {:?}", ratio);
        } else {
            println!("No intersection point found");
        }
        false
    }
}
