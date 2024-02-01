use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;

#[derive(Default)]
pub struct CircularRotation {
    /// The IDs of the robots to move.
    ids: Vec<u8>,
}

impl CircularRotation {
    pub fn new(ids: Vec<u8>) -> Self {
        Self { ids }
    }
}

impl Strategy for CircularRotation {
    fn name(&self) -> &'static str {
        "Circular Rotation"
    }

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        _tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.clear_all();
        let num_robots = self.ids.len();

        for &id in &self.ids {
            // Calculate the index of the next robot in the circular rotation
            let next_id = (id + 1) % num_robots as u8;

            // Get the current and next robot's positions
            let current_robot_position = match world.allies_bot.get(&id) {
                None => continue,
                Some(robot) => robot.pose.position,
            };

            let next_robot_position = match world.allies_bot.get(&(next_id as u8)) {
                None => continue,
                Some(robot) => robot.pose.position,
            };

            // Calculate the angle from current to next robot
            let angle_to_next_robot = (next_robot_position.y - current_robot_position.y)
                .atan2(next_robot_position.x - current_robot_position.x);

            // Move the current robot towards the next robot's position
            action_wrapper.push(id, MoveTo::new(next_robot_position, angle_to_next_robot));
        }

        false
    }
}
