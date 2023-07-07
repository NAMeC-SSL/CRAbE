use std::collections::HashMap;
use nalgebra::{distance, Point2};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, RobotMap, World};
use crate::action::order_raw::RawOrder;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;

struct InitPoseAndStatus {
    status: TestMoveToStatus,
    initial_pose: Point2<f64>,
}

impl Default for InitPoseAndStatus {
    fn default() -> Self {
        InitPoseAndStatus {
            status: TestMoveToStatus::Forward,
            initial_pose: Point2::new(0.0, 0.0),
        }
    }
}

impl InitPoseAndStatus {
    fn new(initial_position: Point2<f64>) -> Self {
        Self {
            status: TestMoveToStatus::Forward,
            initial_pose: initial_position,
        }
    }
}

pub struct TestMoveTo {
    /// IDs of the robots to move, with their current state
    ids_pose_n_status: HashMap<u8, InitPoseAndStatus>,
    /// True if the poses are initialized
    initialized: bool,
}

enum TestMoveToStatus {
    Forward,
    Backwards,
}

impl Default for TestMoveTo {
    fn default() -> Self {
        Self {
            ids_pose_n_status: Default::default(),
            initialized: false,
        }
    }
}

impl TestMoveTo {
    pub fn new(ids: Vec<u8>) -> Self {
        let mut map = HashMap::new();
        ids.iter().for_each(|id| {
                map.insert(id.clone(), InitPoseAndStatus::new(Point2::new(0., 0.)));
            }
        );

        Self {
            ids_pose_n_status: map,
            initialized: false,
        }
    }

    fn initialize_self(&mut self, allies_bot: &RobotMap<AllyInfo>) {
        self.ids_pose_n_status.iter_mut().for_each(|(id, pose_and_status)| {
            if let Some(rob) = allies_bot.get(id) {
                pose_and_status.initial_pose = rob.pose.position;
            }
        });

        let mut initialized = false;
        self.ids_pose_n_status.iter().for_each(|(id, r)| {
            initialized = r.initial_pose != Point2::new(0., 0.);
        });
        self.initialized = initialized;
    }
}

impl Strategy for TestMoveTo {
    fn step(&mut self, world: &World, _: &mut ToolData, action_wrapper: &mut ActionWrapper) -> bool {
        if !self.initialized {
            self.initialize_self(&world.allies_bot);
        }

        self.ids_pose_n_status.iter_mut().for_each(|(id, luigi)| {
            if let Some(robot) = world.allies_bot.get(id) {


                let dist = distance(&luigi.initial_pose, &robot.pose.position);

                dbg!(dist);
                if dist > 1. {
                    luigi.status = TestMoveToStatus::Backwards;
                } else if dist < 0.2 {
                    luigi.status = TestMoveToStatus::Forward;
                }

                action_wrapper.clean(*id);
                let velocity = match luigi.status {
                    TestMoveToStatus::Forward => { 1. }
                    TestMoveToStatus::Backwards => { -1. }
                };

                action_wrapper.push(*id, RawOrder::new(
                    Command {
                        forward_velocity: velocity,
                        ..Default::default()
                    }
                ));
            }
        });
        false
    }

    fn name(&self) -> &'static str { "TestMoveTo" }
}