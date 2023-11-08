use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
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
        for id in &self.ids {
            action_wrapper.push(*id, MoveTo::new(Point2::new(0.0, 0.0), PI));
        }
        // action_wrapper.push(self.id, MoveTo::new(Point2::new(1.0, 1.0), 0));
        // action_wrapper.push(self.id, MoveTo::new(Point2::new(1.0, -1.0), 0));
        // action_wrapper.push(self.id, MoveTo::new(Point2::new(-1.0, -1.0), 0));
        true
    }
}
