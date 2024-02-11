use nalgebra::Point2;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crate::action::ActionWrapper;
use crate::action::bezier_move::BezierMove;
use crate::strategy::Strategy;

pub struct AvoidObstacle {
    id: u8,
}

impl AvoidObstacle {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for AvoidObstacle {
    fn name(&self) -> &'static str { "POC-AvoidObstacle" }

    fn step(&mut self, _world: &World, _tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) -> bool {
        action_wrapper.push(
            self.id,
            // MoveTo::new(Point2::new(0., 0.), 0.)
            BezierMove::new(Point2::new(1., 0.), 0.)
        );
        true
    }
}