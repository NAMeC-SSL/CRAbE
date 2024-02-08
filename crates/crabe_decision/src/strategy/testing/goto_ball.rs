use std::f64::consts::PI;
use crabe_framework::data::{world::World, tool::ToolData};
use nalgebra::Point2;
use crate::{strategy::Strategy, action::{ActionWrapper, move_to::MoveTo}};

const DISTANCE_TO_BALL: f64 = 0.1;
const INACURACY: f64 = 0.01;
const ANGLE_INACURACY: f64 = 0.01;

/// The GotoBall struct represents a strategy that commands a robot to move next to the ball
#[derive(Default)]
pub struct GotoBall {
    /// The id of the robot to move.
    id: u8,
}

impl GotoBall {
    /// Creates a new GotoBall instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for GotoBall {
    fn name(&self) -> &'static str {
        "GotoBall"
    }

    /// Executes the GotoBall strategy.
    ///
    /// This strategy commands the robot with the specified ID to move next to the ball and face it
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
        let ball = match &world.ball {
            Some(b) => b,
            None => {
                eprintln!("Cannot find ball");
                return false
            }
        }.position_2d();
        
        let robot = &match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                eprintln!("Cannot get robot");
                return false
            }
        }.pose;

        let new_pos = (robot.position - ball).normalize()*DISTANCE_TO_BALL;
        let mut angle = f64::atan(new_pos.y/new_pos.x);
        if new_pos.x > 0.0 {
            angle += PI;
        }
        let new_pos = Point2::new(new_pos.x + ball.x, new_pos.y + ball.y);

        action_wrapper.clear(self.id);
        // if (new_pos - robot.position).norm() <= DISTANCE_TO_BALL + INACURACY && angle <= ANGLE_INACURACY {
        //     return true
        // }

        action_wrapper.push(self.id, MoveTo::new(new_pos, angle));
        false
    }
}
