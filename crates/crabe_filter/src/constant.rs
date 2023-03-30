use std::time::Duration;

pub const PACKET_BUFFER_SIZE: usize = 64;
pub const ROBOT_TIMEOUT: Duration = Duration::from_secs(2);
pub const VISION_TIMEOUT: Duration = Duration::from_secs(2);