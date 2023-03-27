use crate::data::FilterData;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;

pub mod vision;
pub mod game_controller;

pub trait PreFilter {
    fn step(
        &mut self,
        inbound_data: &InboundData,
        team_color: &TeamColor,
        filter_data: &mut FilterData,
    );
}
