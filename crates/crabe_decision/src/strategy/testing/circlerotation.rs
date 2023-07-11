use std::time::Instant;

use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::{vectors::{vector_from_angle, angle_to_point}, shape::Circle};
use nalgebra::Point2;


pub struct CircleRotation {
    /// The id of the robot to move.
    id: u8,
    circle: Circle,
    start_time: Instant,
}

impl CircleRotation {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8, circle: Circle) -> Self {
        Self { id , circle, start_time: Instant::now()}
    }
}

impl Strategy for CircleRotation {
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
        let elapsed = (self.start_time.elapsed().as_millis() as f64)/1000.;
        let sin = f64::sin(elapsed);
        let cos = f64::cos(elapsed);
        let robot = match world.allies_bot.get(&self.id) {
            None => {
                return false;
            }
            Some(robot) => {
                robot
            }
        };  
        action_wrapper.push(
            self.id,
            MoveTo::new(
                dbg!(Point2::new(
                    sin * self.circle.radius,
                    cos * self.circle.radius,
                )),
                angle_to_point(self.circle.center, robot.pose.position),
                0.0, None,
                false, false,
            ),
        );
        true
    }
    fn name(&self) -> &'static str {
        return "GotoCenter";
    }
}
