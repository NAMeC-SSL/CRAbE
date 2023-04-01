use std::time::Duration;
use chrono::{DateTime, Utc};
use ringbuffer::RingBufferExt;
use crabe_framework::data::world::World;
use crate::constant;
use crate::data::{FilterData, TrackedRobotMap};
use crate::post_filter::PostFilter;

pub struct FlipSideFilter;

impl FlipSideFilter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for FlipSideFilter {
    fn default() -> Self {
        Self {}
    }
}

impl PostFilter for FlipSideFilter {
    fn step(&mut self, filter_data: &mut FilterData,  world: &World) {
        dbg!(world.team_color);
        dbg!(world.data.positive_half);
        if world.team_color == world.data.positive_half {
            filter_data.allies.iter_mut().for_each(|(_id, r)| {
                r.packets.iter_mut().position()
            };
        }
    }
}