/// Size of the buffer for packet (game_controller, vision, etc...).
/// This buffer size was chosen to accommodate the largest possible packet size
/// for the protocols that use it, including overhead and padding.
pub const BUFFER_SIZE: usize = 4096;
pub const VISION_PORT_REAL: u16 = 10006;
pub const VISION_PORT_SIM: u16 = 10020;
pub const SIM_PORT_BLUE: u16 = 10301;
pub const SIM_PORT_YELLOW: u16 = 10302;
pub const TRACKED_PORT: u16 = 10010;
