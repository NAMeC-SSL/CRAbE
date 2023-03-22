use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;
use crabe_framework::data::output::Kick;
use crate::action::move_with_kick::{MoveToWithKick};

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Mbappe {
    /// The id of the robot to move.
    id: u8,
}

impl Mbappe {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for Mbappe {
    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        let should_kick = |w: &World, id: u8, t: &ToolData| {
            if let Some(r) = w.allies_bot.get(&id) {
                if let Some(b) = &w.ball {
                    if (b.position.xy() - r.pose.position).norm() < 0.1 {
                        return Some(Kick::StraightKick {
                            power: 0.5,
                        })
                    }
                }


            }
            None
        };

        action_wrapper.push(self.id, MoveToWithKick::new(Point2::new(-1.0, 2.0), 0.0, should_kick));
        false
    }
}
