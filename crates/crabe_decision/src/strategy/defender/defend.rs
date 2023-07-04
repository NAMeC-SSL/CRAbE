use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::shape::Line;
use crabe_math::vectors;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The penaltyPrepKeeper struct represents a strategy that commands the keeper to set in the penalty formation
/// It is used when there is a penalty for the opponent team
#[derive(Default)]
pub struct Defend {
    /// The id of the robot to move.
    id: u8,
    left: bool,//defense the ball with 2 bot
}

impl Defend {
    /// Creates a new penaltyPrepKeeper instance with the desired robot id.
    pub fn new(id: u8, left: bool) -> Self {
        Self { id , left}
    }
}

impl Strategy for Defend {
    /// Executes the penaltyPrepKeeper strategy.
    ///
    /// This strategy commands all the robots to move in position for
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
                action_wrapper.push(self.id, MoveTo::new(robot.pose.position, robot.pose.orientation, 0., None,false,true));
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        let shoot_line = Line::new(ball_pos, Point2::new(-world.geometry.field.length/2.,0.));
        let shoot_dir = shoot_line.start - shoot_line.end;
        let bot_line_pos_side_point = Point2::new(-world.geometry.field.length/2. + world.geometry.ally_penalty.depth, -world.geometry.field.width/2.);
        let bot_line_pos = Line::new(bot_line_pos_side_point, Point2::new(bot_line_pos_side_point.x, -bot_line_pos_side_point.y));
        let interseption_point = shoot_line.intersection_line(&bot_line_pos);
        let perpendicular_dir = vectors::rotate_vector(shoot_dir, PI/2.).normalize() * 0.109;
        if let Some(interseption_position) = interseption_point{
            let mut final_pos = interseption_position;
            if self.left {final_pos = final_pos + perpendicular_dir;}
            else {final_pos = final_pos - perpendicular_dir}
            let to_ball_angle = vectors::angle_to_point(ball_pos, robot.pose.position);
            action_wrapper.push(self.id, MoveTo::new(final_pos, to_ball_angle, 0., None,false,true));
        }
        false
    }
}

