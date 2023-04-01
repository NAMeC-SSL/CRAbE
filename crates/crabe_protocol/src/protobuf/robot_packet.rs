#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Kicker {
    NoKick = 0,
    Flat = 1,
    Chip = 2,
}
impl Kicker {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Kicker::NoKick => "NO_KICK",
            Kicker::Flat => "FLAT",
            Kicker::Chip => "CHIP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_KICK" => Some(Self::NoKick),
            "FLAT" => Some(Self::Flat),
            "CHIP" => Some(Self::Chip),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseCommand {
    /// The unique ID of the robot, as identified by SSL-Vision.
    #[prost(uint32, tag = "1")]
    pub robot_id: u32,
    /// Desired forward drive velocity in meters / second.
    #[prost(float, tag = "2")]
    pub normal_velocity: f32,
    /// Desired sideways left drive velocity in meters / second.
    #[prost(float, tag = "3")]
    pub tangential_velocity: f32,
    /// Desired counter-clockwise angular velocity in radians / second.
    #[prost(float, tag = "4")]
    pub angular_velocity: f32,
    #[prost(enumeration = "Kicker", tag = "5")]
    pub kick: i32,
    #[prost(float, tag = "6")]
    pub kick_power: f32,
    #[prost(bool, tag = "8")]
    pub charge: bool,
    #[prost(float, tag = "9")]
    pub dribbler: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseFeedback {
    /// Robot identifier
    #[prost(uint32, tag = "1")]
    pub robot_id: u32,
    #[prost(float, tag = "2")]
    pub motor_1_speed: f32,
    #[prost(float, tag = "3")]
    pub motor_2_speed: f32,
    #[prost(float, tag = "4")]
    pub motor_3_speed: f32,
    #[prost(float, tag = "5")]
    pub motor_4_speed: f32,
    #[prost(float, tag = "6")]
    pub voltage: f32,
    #[prost(bool, tag = "7")]
    pub ir: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PcToBase {
    #[prost(message, repeated, tag = "1")]
    pub commands: ::prost::alloc::vec::Vec<BaseCommand>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseToPc {
    #[prost(message, repeated, tag = "1")]
    pub feedbacks: ::prost::alloc::vec::Vec<BaseFeedback>,
}
