use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;
use crate::data::FilterData;
use crate::pre_filter::PreFilter;

pub struct GameControllerFilter;

impl GameControllerFilter {
    fn new() -> Self {
        Self
    }
}

impl PreFilter for GameControllerFilter {
    fn step(
        &mut self,
        inbound_data: &InboundData,
        _team_color: &TeamColor,
        filter_data: &mut FilterData,
    ) {
        filter_data.referee.extend(inbound_data.gc_packet.iter());
    }
}