use chrono::{DateTime, Utc};
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;
use crate::data::FilterData;
use crate::pre_filter::PreFilter;

pub struct TickFilter;
impl TickFilter {
    pub fn new() -> TickFilter {
        TickFilter
    }
}
impl PreFilter for TickFilter {
    fn step(&mut self, _inbound_data: &InboundData, _team_color: &TeamColor, filter_data: &mut FilterData) {
        filter_data.tick += 1;
        filter_data.timestamp = Utc::now();
    }
}