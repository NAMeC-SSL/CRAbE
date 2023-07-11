use std::ops::Mul;

use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::output::Kick;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors::{self};

pub struct Passer {
    /// The id of the robot to move.
    id: u8,
    state: PasserState
}
impl Passer {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, state: PasserState::PlaceForPass}
    }
}
enum PasserState{
    PlaceForPass,
    Pass
}

impl Strategy for Passer {
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
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        let robot_pos = robot.pose.position;
        let robot_to_ball = ball_pos - robot_pos;
        let robot_current_dir = vectors::vector_from_angle(robot.pose.orientation);
        let dot_with_ball = robot_current_dir.normalize().dot(&robot_to_ball.normalize());
        match self.state {
            PasserState::PlaceForPass => {
                if dot_with_ball > 0.9{
                    self.state = PasserState::Pass
                }
                action_wrapper.push(self.id, MoveTo::new(ball_pos + (robot_pos - ball_pos).normalize().mul(0.3), vectors::angle_to_point(ball_pos, robot_pos), 0., None, false, false));
            },
            PasserState::Pass => {
                let dist_to_ball = robot_to_ball.norm();
                let kick: Option<Kick> = if dist_to_ball < 0.125 && dot_with_ball > 0.9{
                    Some(Kick::StraightKick {  power: 4. }) 
                }else {None};
                action_wrapper.push(self.id, MoveTo::new(ball_pos, vectors::angle_to_point(ball_pos, robot_pos), 1.,  kick, false, true));
                if dot_with_ball < 0.9  || dist_to_ball > 0.4{
                    self.state = PasserState::PlaceForPass;
                }
            }
        };
        false
    }

    fn name(&self) -> &'static str{
        return "Passer";
    }
}
