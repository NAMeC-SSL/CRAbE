use std::time::Duration;

pub const PACKET_BUFFER_SIZE: usize = 64;
pub const ROBOT_VISION_TIMEOUT: Duration = Duration::from_secs(2);
pub const KICK_TIMEOUT: Duration = Duration::from_secs(10);
pub const PENALTY_TIMEOUT: Duration = Duration::from_secs(10);