use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::constant;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::{Field, Point2};
use std::f64::consts::PI;

#[derive(Default)]
pub struct GoToMiddle {
    /// The id of the robot to move.
    ids: Vec<u8>,
}

impl GoToMiddle {
    pub fn new(ids: Vec<u8>) -> Self {
        Self { ids }
    }
}

impl Strategy for GoToMiddle {
    fn name(&self) -> &'static str {
        "Go to Middle"
    }

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        let goal_width = world.geometry.ally_goal.width;
        let num_robots = self.ids.len();
        let spacing_between_robots = 1;

        for id in 0..num_robots {
            let y_position = goal_width
                + spacing_between_robots as f64 * (id as f64 - (num_robots - 1) as f64 / 2.0);
            action_wrapper.push(id as u8, MoveTo::new(Point2::new(0.5, y_position), PI));
            action_wrapper.push(id as u8, MoveTo::new(Point2::new(-4.5, y_position), PI));
        }
        true
    }
}
