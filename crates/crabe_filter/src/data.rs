pub mod camera;

use crate::constant;
use crate::data::camera::{CamBall, CamGeometry, CamRobot};
use chrono::{DateTime, Utc};
use constant::PACKET_BUFFER_SIZE;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Robot};
use ringbuffer::ConstGenericRingBuffer;
use std::collections::HashMap;
use std::time::Instant;
use crate::data::referee::Referee;

#[derive(Clone, Debug)]
pub struct FrameInfo {
    pub camera_id: u32,
    pub frame_number: u32,
    pub t_capture: DateTime<Utc>,
}

pub mod referee;

pub type TrackedRobotMap<T> = HashMap<u8, TrackedRobot<T>>;

pub struct FilterData {
    pub allies: TrackedRobotMap<AllyInfo>,
    pub enemies: TrackedRobotMap<EnemyInfo>,
    pub ball: TrackedBall,
    pub geometry: CamGeometry,
    pub referee: Vec<Referee>,
}

pub struct TrackedRobot<T> {
    pub packets: ConstGenericRingBuffer<CamRobot, PACKET_BUFFER_SIZE>,
    pub data: Robot<T>,
    pub last_update: DateTime<Utc>,
}

impl<T: Default> Default for TrackedRobot<T> {
    fn default() -> Self {
        TrackedRobot {
            packets: ConstGenericRingBuffer::new(),
            data: Robot::<T>::default(),
            last_update: Utc::now(),
        }
    }
}

pub struct TrackedBall {
    pub packets: ConstGenericRingBuffer<CamBall, PACKET_BUFFER_SIZE>,
    pub data: Ball,
    pub last_update: Instant,
}

impl Default for TrackedBall {
    fn default() -> Self {
        Self {
            packets: ConstGenericRingBuffer::new(),
            last_update: Instant::now(),
            data: Default::default(),
        }
    }
}
