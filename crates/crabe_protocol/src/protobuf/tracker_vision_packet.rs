/// A vector with two dimensions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector2 {
    #[prost(float, required, tag = "1")]
    pub x: f32,
    #[prost(float, required, tag = "2")]
    pub y: f32,
}
/// A vector with three dimensions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector3 {
    #[prost(float, required, tag = "1")]
    pub x: f32,
    #[prost(float, required, tag = "2")]
    pub y: f32,
    #[prost(float, required, tag = "3")]
    pub z: f32,
}
/// A unique robot id with team information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotId {
    /// The robot number
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    /// The team color
    #[prost(enumeration = "TeamColor", required, tag = "2")]
    pub team_color: i32,
}
/// A single tracked ball
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackedBall {
    /// The position (x, y, height) \[m\] in the ssl-vision coordinate system
    #[prost(message, required, tag = "1")]
    pub pos: Vector3,
    /// The velocity \[m/s\] in the ssl-vision coordinate system
    #[prost(message, optional, tag = "2")]
    pub vel: ::core::option::Option<Vector3>,
    /// The visibility of the ball
    /// A value between 0 (not visible) and 1 (visible)
    /// The exact implementation depends on the source software
    #[prost(float, optional, tag = "3")]
    pub visibility: ::core::option::Option<f32>,
}
/// A ball kicked by a robot, including predictions when the ball will come to a stop
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KickedBall {
    /// The initial position \[m\] from which the ball was kicked
    #[prost(message, required, tag = "1")]
    pub pos: Vector2,
    /// The initial velocity \[m/s\] with which the ball was kicked
    #[prost(message, required, tag = "2")]
    pub vel: Vector3,
    /// The unix timestamp \[s\] when the kick was performed
    #[prost(double, required, tag = "3")]
    pub start_timestamp: f64,
    /// The predicted unix timestamp \[s\] when the ball comes to a stop
    #[prost(double, optional, tag = "4")]
    pub stop_timestamp: ::core::option::Option<f64>,
    /// The predicted position \[m\] at which the ball will come to a stop
    #[prost(message, optional, tag = "5")]
    pub stop_pos: ::core::option::Option<Vector2>,
    /// The robot that kicked the ball
    #[prost(message, optional, tag = "6")]
    pub robot_id: ::core::option::Option<RobotId>,
}
/// A single tracked robot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackedRobot {
    #[prost(message, required, tag = "1")]
    pub robot_id: RobotId,
    /// The position \[m\] in the ssl-vision coordinate system
    #[prost(message, required, tag = "2")]
    pub pos: Vector2,
    /// The orientation \[rad\] in the ssl-vision coordinate system
    #[prost(float, required, tag = "3")]
    pub orientation: f32,
    /// The velocity \[m/s\] in the ssl-vision coordinate system
    #[prost(message, optional, tag = "4")]
    pub vel: ::core::option::Option<Vector2>,
    /// The angular velocity \[rad/s\] in the ssl-vision coordinate system
    #[prost(float, optional, tag = "5")]
    pub vel_angular: ::core::option::Option<f32>,
    /// The visibility of the robot
    /// A value between 0 (not visible) and 1 (visible)
    /// The exact implementation depends on the source software
    #[prost(float, optional, tag = "6")]
    pub visibility: ::core::option::Option<f32>,
}
/// A frame that contains all currently tracked objects on the field on all cameras
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackedFrame {
    /// A monotonous increasing frame counter
    #[prost(uint32, required, tag = "1")]
    pub frame_number: u32,
    /// The unix timestamp in \[s\] of the data
    #[prost(double, required, tag = "2")]
    pub timestamp: f64,
    /// The list of detected balls
    /// The first ball is the primary one
    /// Sources may add additional balls based on their capabilities
    #[prost(message, repeated, tag = "3")]
    pub balls: ::prost::alloc::vec::Vec<TrackedBall>,
    /// The list of detected robots of both teams
    #[prost(message, repeated, tag = "4")]
    pub robots: ::prost::alloc::vec::Vec<TrackedRobot>,
    /// Information about a kicked ball, if the ball was kicked by a robot and is still moving
    /// Note: This field is optional. Some source implementations might not set this at any time
    #[prost(message, optional, tag = "5")]
    pub kicked_ball: ::core::option::Option<KickedBall>,
    /// List of capabilities of the source implementation
    #[prost(enumeration = "Capability", repeated, packed = "false", tag = "6")]
    pub capabilities: ::prost::alloc::vec::Vec<i32>,
}
/// The team color of the robot
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TeamColor {
    /// team not set
    Unknown = 0,
    /// yellow team
    Yellow = 1,
    /// blue team
    Blue = 2,
}
impl TeamColor {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TeamColor::Unknown => "TEAM_COLOR_UNKNOWN",
            TeamColor::Yellow => "TEAM_COLOR_YELLOW",
            TeamColor::Blue => "TEAM_COLOR_BLUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEAM_COLOR_UNKNOWN" => Some(Self::Unknown),
            "TEAM_COLOR_YELLOW" => Some(Self::Yellow),
            "TEAM_COLOR_BLUE" => Some(Self::Blue),
            _ => None,
        }
    }
}
/// Capabilities that a source implementation can have
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Capability {
    Unknown = 0,
    DetectFlyingBalls = 1,
    DetectMultipleBalls = 2,
    DetectKickedBalls = 3,
}
impl Capability {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Capability::Unknown => "CAPABILITY_UNKNOWN",
            Capability::DetectFlyingBalls => "CAPABILITY_DETECT_FLYING_BALLS",
            Capability::DetectMultipleBalls => "CAPABILITY_DETECT_MULTIPLE_BALLS",
            Capability::DetectKickedBalls => "CAPABILITY_DETECT_KICKED_BALLS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CAPABILITY_UNKNOWN" => Some(Self::Unknown),
            "CAPABILITY_DETECT_FLYING_BALLS" => Some(Self::DetectFlyingBalls),
            "CAPABILITY_DETECT_MULTIPLE_BALLS" => Some(Self::DetectMultipleBalls),
            "CAPABILITY_DETECT_KICKED_BALLS" => Some(Self::DetectKickedBalls),
            _ => None,
        }
    }
}
/// A wrapper packet containing meta data of the source
/// Also serves for the possibility to extend the protocol later
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackerWrapperPacket {
    /// A random UUID of the source that is kept constant at the source while running
    /// If multiple sources are broadcasting to the same network, this id can be used to identify individual sources
    #[prost(string, required, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    /// The name of the source software that is producing this messages.
    #[prost(string, optional, tag = "2")]
    pub source_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The tracked frame
    #[prost(message, optional, tag = "3")]
    pub tracked_frame: ::core::option::Option<TrackedFrame>,
}
