use std::f64::consts::{FRAC_PI_6, FRAC_PI_8};
use std::ops::Div;
use nalgebra::{distance, Point2};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crate::action::ActionWrapper;
use crate::action::move_to::MoveTo;
use crate::strategy::Strategy;

const DIST_TARGET_REACHED: f64 = 0.1;

#[derive(Debug)]
enum TestVisionMoveToStatus {
    Placement,
    MovingForward,
    MovingBackwards,
}

pub struct TestVisionMoveTo {
    ids: Vec<u8>,
    status: TestVisionMoveToStatus,
    positive_half: bool,
}

impl TestVisionMoveTo {
    pub fn new(ids: Vec<u8>, positive_half: bool) -> Self {
        Self {
            ids,
            status: TestVisionMoveToStatus::Placement,
            positive_half,
        }
    }


}

impl Strategy for TestVisionMoveTo {
    fn name(&self) -> &'static str {
        "TestVisionMoveTo"
    }

    fn step(&mut self, world: &World, _: &mut ToolData, action_wrapper: &mut ActionWrapper) -> bool {
        // WARNING : Not clearing the action_wrapper leads to stuttering
        action_wrapper.clear_all();
        let sign = if self.positive_half { 1. } else { -1. };
        let mut y_target;
        let mut next_status = TestVisionMoveToStatus::Placement;
        match self.status {
            TestVisionMoveToStatus::Placement => {
                y_target = 0.;
                let allies_placed =
                    world.allies_bot
                        .iter()
                        .filter(|(id, rob)|
                            self.ids.contains(id) && rob.pose.position.y <= DIST_TARGET_REACHED
                        ).count();

                if allies_placed == self.ids.len() {
                    next_status = TestVisionMoveToStatus::MovingForward
                }

            }
            TestVisionMoveToStatus::MovingForward => {
                y_target = 1.;
                next_status = TestVisionMoveToStatus::MovingBackwards;
            }
            TestVisionMoveToStatus::MovingBackwards => {
                y_target = -1.;
                next_status = TestVisionMoveToStatus::MovingForward;
            }
        }

        // Move robots
        let mut change_status = true;

        world.allies_bot.iter()
            .filter(|(ally_id, _)| self.ids.contains(ally_id))
            .for_each(|(ally_id, ally_info)| {
                let target = Point2::new((*ally_id as f64).div(2.) * sign, y_target);
                action_wrapper.push(*ally_id, MoveTo::new(target, FRAC_PI_6 * (*ally_id as f64)));
                change_status = change_status && distance(&target, &ally_info.pose.position) <= DIST_TARGET_REACHED
            });
        if change_status {
            self.status = next_status;
        }

        false
    }
}