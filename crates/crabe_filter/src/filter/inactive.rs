use std::time::Duration;
use chrono::{DateTime, Utc};
use crabe_framework::data::world::World;
use crate::constant;
use crate::data::{FilterData, TrackedRobotMap};
use crate::filter::Filter;

pub struct InactiveFilter {
    timeout: Duration
}

impl InactiveFilter {
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout
        }
    }

    fn purge_inactive<T>(&self, tracked_robots: &mut TrackedRobotMap<T>, now: DateTime<Utc>) {
        tracked_robots.retain(|_id, robot| {
            // Use std duration as chrono does not support const fn yet
            (now - robot.last_update).to_std().map_or(false, |d| d < self.timeout)
        });
    }
}

impl Default for InactiveFilter {
    fn default() -> Self {
        Self {
            timeout: constant::ROBOT_TIMEOUT
        }
    }
}

impl Filter for InactiveFilter {
    fn step(&mut self, filter_data: &mut FilterData, _world: &World) {
        let now = Utc::now();
        self.purge_inactive(&mut filter_data.allies, now);
        self.purge_inactive(&mut filter_data.enemies, now);
    }
}