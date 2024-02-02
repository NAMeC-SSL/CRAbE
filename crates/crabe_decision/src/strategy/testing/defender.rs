use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
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

    fn line(
        &mut self,
        world: &World,
        x: f64
    ) -> Point2<f64> {
        let enlarged_penalty = world.geometry.ally_penalty.enlarged_penalty(0.3);
        return enlarged_penalty.front_line.start;
        // let pen_pos = world.geometry.ally_penalty.top_left_position;
        // let offset = 0.3;
        // let penx = pen_pos.x;
        // let peny = pen_pos.y-offset;
        // let width = world.geometry.ally_penalty.width+offset;
        // let depth = world.geometry.ally_penalty.depth+offset;
        // let tot_length = depth * 2. + width;
        // let dist_along_penalty_line = tot_length * x;
        // if dist_along_penalty_line < depth{
        //     return Point2::new(penx + dist_along_penalty_line , peny );
        // }else if dist_along_penalty_line < depth + width{
        //     return Point2::new(penx + depth, peny- (depth- dist_along_penalty_line));
        // }else{
        //     return Point2::new(penx +depth - (dist_along_penalty_line - (depth + width))  , -peny );
        // }
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
        let current_time = SystemTime::now();
        let mut x = 0.;
        if let Ok(duration) = current_time.duration_since(UNIX_EPOCH) {
            let current_time_ms = duration.as_millis() as f64;
            x = current_time_ms ;
        } 

        let ball_pos = match world.ball.clone() {
            None => {return false;}
            Some(ball) => {ball.position.xy() }
        };

        //TODO add this constant in the geometry (see code in rbc branches maybe)
        let goal_center = world.geometry.ally_goal.front_line.middle();
        let ball_to_goal = Line::new(goal_center, ball_pos);
    
        println!("{:?}", ball_to_goal.intersection_line(&world.geometry.ally_penalty.front_line));
        let oscillating_value = (0.00005 * 2.0 * std::f64::consts::PI * x).sin() * 0.5 + 0.5;
        let pos = self.line(world, oscillating_value);
        action_wrapper.push(self.id, MoveTo::new(pos, 0.));
        false
    }

}
