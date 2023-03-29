use std::time::Duration;
use chrono::{DateTime, Utc};
use crabe_framework::data::world::World;
use crate::constant;
use crate::data::FilterData;
use crate::post_filter::PostFilter;

pub struct VisionTimeoutFilter {
    timeout: Duration,
}

impl VisionTimeoutFilter {
    fn new(timeout: Duration) -> VisionTimeoutFilter {
        VisionTimeoutFilter {
            timeout
        }
    }
}

impl Default for VisionTimeoutFilter {
    fn default() -> Self {
        Self {
            timeout: constant::VISION_TIMEOUT
        }
    }
}

impl PostFilter for VisionTimeoutFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        world.vision_timeout = ((filter_data.timestamp - filter_data.last_vision_received).to_std())
            .map_or(false, |d| d > self.timeout);
    }
}