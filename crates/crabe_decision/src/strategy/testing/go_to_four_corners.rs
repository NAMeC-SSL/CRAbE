use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

#[derive(Default)]
pub struct GoToFourCorners {
    /// The id of the robot to move.
    id: u8,
}

impl GoToFourCorners {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for GoToFourCorners {
    fn name(&self) -> &'static str {
        "GoToFourCorners"
    }

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.push(
            self.id,
            MoveTo::new(
                Point2::new(world.geometry.boundary_width, world.geometry.field.width),
                -PI / 4.0,
            ),
        );
        // action_wrapper.push(self.id, MoveTo::new(Point2::new(-1.0, 1.0), -PI / 4.0));
        // action_wrapper.push(self.id, MoveTo::new(Point2::new(1.0, 1.0), -3.0 * PI / 4.0));
        // action_wrapper.push(self.id, MoveTo::new(Point2::new(1.0, -1.0), 3.0 * PI / 4.0));
        // action_wrapper.push(self.id, MoveTo::new(Point2::new(-1.0, -1.0), PI / 4.0));
        true
    }
}
