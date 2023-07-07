use chrono::{DateTime, Utc};
use nalgebra::{OPoint, Point2, Vector2};
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::{Pose, Robot, RobotVelocity, TeamColor};
use crabe_protocol::protobuf::tracker_vision_packet::{TrackedFrame, TrackedRobot};
use crate::data::FilterData;
use crate::pre_filter::PreFilter;

pub struct TrackerFilter;

impl TrackerFilter {
    fn new() -> TrackerFilter {
        TrackerFilter
    }
}

impl PreFilter for TrackerFilter {
    fn step(&mut self, inbound_data: &mut InboundData, team_color: &TeamColor, filter_data: &mut FilterData) {
        filter_data.tracker = inbound_data.tracker_packet.drain(..).collect()
    }
}