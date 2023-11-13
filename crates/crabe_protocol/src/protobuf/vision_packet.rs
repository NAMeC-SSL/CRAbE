#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslDetectionBall {
    #[prost(float, required, tag = "1")]
    pub confidence: f32,
    #[prost(uint32, optional, tag = "2")]
    pub area: ::core::option::Option<u32>,
    #[prost(float, required, tag = "3")]
    pub x: f32,
    #[prost(float, required, tag = "4")]
    pub y: f32,
    #[prost(float, optional, tag = "5")]
    pub z: ::core::option::Option<f32>,
    #[prost(float, required, tag = "6")]
    pub pixel_x: f32,
    #[prost(float, required, tag = "7")]
    pub pixel_y: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslDetectionRobot {
    #[prost(float, required, tag = "1")]
    pub confidence: f32,
    #[prost(uint32, optional, tag = "2")]
    pub robot_id: ::core::option::Option<u32>,
    #[prost(float, required, tag = "3")]
    pub x: f32,
    #[prost(float, required, tag = "4")]
    pub y: f32,
    #[prost(float, optional, tag = "5")]
    pub orientation: ::core::option::Option<f32>,
    #[prost(float, required, tag = "6")]
    pub pixel_x: f32,
    #[prost(float, required, tag = "7")]
    pub pixel_y: f32,
    #[prost(float, optional, tag = "8")]
    pub height: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslDetectionFrame {
    #[prost(uint32, required, tag = "1")]
    pub frame_number: u32,
    #[prost(double, required, tag = "2")]
    pub t_capture: f64,
    #[prost(double, required, tag = "3")]
    pub t_sent: f64,
    #[prost(uint32, required, tag = "4")]
    pub camera_id: u32,
    #[prost(message, repeated, tag = "5")]
    pub balls: ::prost::alloc::vec::Vec<SslDetectionBall>,
    #[prost(message, repeated, tag = "6")]
    pub robots_yellow: ::prost::alloc::vec::Vec<SslDetectionRobot>,
    #[prost(message, repeated, tag = "7")]
    pub robots_blue: ::prost::alloc::vec::Vec<SslDetectionRobot>,
}
/// A 2D float vector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector2f {
    /// X-coordinate in mm
    #[prost(float, required, tag = "1")]
    pub x: f32,
    /// Y-coordinate in mm
    #[prost(float, required, tag = "2")]
    pub y: f32,
}
/// Represents a field marking as a line segment represented by a start point p1,
/// and end point p2, and a line thickness. The start and end points are along
/// the center of the line, so the thickness of the line extends by thickness / 2
/// on either side of the line.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslFieldLineSegment {
    /// Name of this field marking.
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Start point of the line segment.
    #[prost(message, required, tag = "2")]
    pub p1: Vector2f,
    /// End point of the line segment.
    #[prost(message, required, tag = "3")]
    pub p2: Vector2f,
    /// Thickness of the line segment.
    #[prost(float, required, tag = "4")]
    pub thickness: f32,
    /// The type of this shape
    #[prost(enumeration = "SslFieldShapeType", optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
}
/// Represents a field marking as a circular arc segment represented by center point, a
/// start angle, an end angle, and an arc thickness.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslFieldCircularArc {
    /// Name of this field marking.
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Center point of the circular arc.
    #[prost(message, required, tag = "2")]
    pub center: Vector2f,
    /// Radius of the arc.
    #[prost(float, required, tag = "3")]
    pub radius: f32,
    /// Start angle in counter-clockwise order.
    #[prost(float, required, tag = "4")]
    pub a1: f32,
    /// End angle in counter-clockwise order.
    #[prost(float, required, tag = "5")]
    pub a2: f32,
    /// Thickness of the arc.
    #[prost(float, required, tag = "6")]
    pub thickness: f32,
    /// The type of this shape
    #[prost(enumeration = "SslFieldShapeType", optional, tag = "7")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslGeometryFieldSize {
    /// Field length (distance between goal lines) in mm
    #[prost(int32, required, tag = "1")]
    pub field_length: i32,
    /// Field width (distance between touch lines) in mm
    #[prost(int32, required, tag = "2")]
    pub field_width: i32,
    /// Goal width (distance between inner edges of goal posts) in mm
    #[prost(int32, required, tag = "3")]
    pub goal_width: i32,
    /// Goal depth (distance from outer goal line edge to inner goal back) in mm
    #[prost(int32, required, tag = "4")]
    pub goal_depth: i32,
    /// Boundary width (distance from touch/goal line centers to boundary walls) in mm
    #[prost(int32, required, tag = "5")]
    pub boundary_width: i32,
    /// Generated line segments based on the other parameters
    #[prost(message, repeated, tag = "6")]
    pub field_lines: ::prost::alloc::vec::Vec<SslFieldLineSegment>,
    /// Generated circular arcs based on the other parameters
    #[prost(message, repeated, tag = "7")]
    pub field_arcs: ::prost::alloc::vec::Vec<SslFieldCircularArc>,
    /// Depth of the penalty/defense area (measured between line centers) in mm
    #[prost(int32, optional, tag = "8")]
    pub penalty_area_depth: ::core::option::Option<i32>,
    /// Width of the penalty/defense area (measured between line centers) in mm
    #[prost(int32, optional, tag = "9")]
    pub penalty_area_width: ::core::option::Option<i32>,
    /// Radius of the center circle (measured between line centers) in mm
    #[prost(int32, optional, tag = "10")]
    pub center_circle_radius: ::core::option::Option<i32>,
    /// Thickness/width of the lines on the field in mm
    #[prost(int32, optional, tag = "11")]
    pub line_thickness: ::core::option::Option<i32>,
    /// Distance between the goal center and the center of the penalty mark in mm
    #[prost(int32, optional, tag = "12")]
    pub goal_center_to_penalty_mark: ::core::option::Option<i32>,
    /// Goal height in mm
    #[prost(int32, optional, tag = "13")]
    pub goal_height: ::core::option::Option<i32>,
    /// Ball radius in mm (note that this is a float type to represent sub-mm precision)
    #[prost(float, optional, tag = "14")]
    pub ball_radius: ::core::option::Option<f32>,
    /// Max allowed robot radius in mm (note that this is a float type to represent sub-mm precision)
    #[prost(float, optional, tag = "15")]
    pub max_robot_radius: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslGeometryCameraCalibration {
    #[prost(uint32, required, tag = "1")]
    pub camera_id: u32,
    #[prost(float, required, tag = "2")]
    pub focal_length: f32,
    #[prost(float, required, tag = "3")]
    pub principal_point_x: f32,
    #[prost(float, required, tag = "4")]
    pub principal_point_y: f32,
    #[prost(float, required, tag = "5")]
    pub distortion: f32,
    #[prost(float, required, tag = "6")]
    pub q0: f32,
    #[prost(float, required, tag = "7")]
    pub q1: f32,
    #[prost(float, required, tag = "8")]
    pub q2: f32,
    #[prost(float, required, tag = "9")]
    pub q3: f32,
    #[prost(float, required, tag = "10")]
    pub tx: f32,
    #[prost(float, required, tag = "11")]
    pub ty: f32,
    #[prost(float, required, tag = "12")]
    pub tz: f32,
    #[prost(float, optional, tag = "13")]
    pub derived_camera_world_tx: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "14")]
    pub derived_camera_world_ty: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "15")]
    pub derived_camera_world_tz: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "16")]
    pub pixel_image_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "17")]
    pub pixel_image_height: ::core::option::Option<u32>,
}
/// Two-Phase model for straight-kicked balls.
/// There are two phases with different accelerations during the ball kicks:
/// 1. Sliding
/// 2. Rolling
/// The full model is described in the TDP of ER-Force from 2016, which can be found here:
/// <https://ssl.robocup.org/wp-content/uploads/2019/01/2016_ETDP_ER-Force.pdf>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslBallModelStraightTwoPhase {
    /// Ball sliding acceleration \[m/s^2\] (should be negative)
    #[prost(double, required, tag = "1")]
    pub acc_slide: f64,
    /// Ball rolling acceleration \[m/s^2\] (should be negative)
    #[prost(double, required, tag = "2")]
    pub acc_roll: f64,
    /// Fraction of the initial velocity where the ball starts to roll
    #[prost(double, required, tag = "3")]
    pub k_switch: f64,
}
/// Fixed-Loss model for chipped balls.
/// Uses fixed damping factors for xy and z direction per hop.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslBallModelChipFixedLoss {
    /// Chip kick velocity damping factor in XY direction for the first hop
    #[prost(double, required, tag = "1")]
    pub damping_xy_first_hop: f64,
    /// Chip kick velocity damping factor in XY direction for all following hops
    #[prost(double, required, tag = "2")]
    pub damping_xy_other_hops: f64,
    /// Chip kick velocity damping factor in Z direction for all hops
    #[prost(double, required, tag = "3")]
    pub damping_z: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslGeometryModels {
    #[prost(message, optional, tag = "1")]
    pub straight_two_phase: ::core::option::Option<SslBallModelStraightTwoPhase>,
    #[prost(message, optional, tag = "2")]
    pub chip_fixed_loss: ::core::option::Option<SslBallModelChipFixedLoss>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslGeometryData {
    #[prost(message, required, tag = "1")]
    pub field: SslGeometryFieldSize,
    #[prost(message, repeated, tag = "2")]
    pub calib: ::prost::alloc::vec::Vec<SslGeometryCameraCalibration>,
    #[prost(message, optional, tag = "3")]
    pub models: ::core::option::Option<SslGeometryModels>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SslFieldShapeType {
    Undefined = 0,
    CenterCircle = 1,
    TopTouchLine = 2,
    BottomTouchLine = 3,
    LeftGoalLine = 4,
    RightGoalLine = 5,
    HalfwayLine = 6,
    CenterLine = 7,
    LeftPenaltyStretch = 8,
    RightPenaltyStretch = 9,
    LeftFieldLeftPenaltyStretch = 10,
    LeftFieldRightPenaltyStretch = 11,
    RightFieldLeftPenaltyStretch = 12,
    RightFieldRightPenaltyStretch = 13,
}
impl SslFieldShapeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SslFieldShapeType::Undefined => "Undefined",
            SslFieldShapeType::CenterCircle => "CenterCircle",
            SslFieldShapeType::TopTouchLine => "TopTouchLine",
            SslFieldShapeType::BottomTouchLine => "BottomTouchLine",
            SslFieldShapeType::LeftGoalLine => "LeftGoalLine",
            SslFieldShapeType::RightGoalLine => "RightGoalLine",
            SslFieldShapeType::HalfwayLine => "HalfwayLine",
            SslFieldShapeType::CenterLine => "CenterLine",
            SslFieldShapeType::LeftPenaltyStretch => "LeftPenaltyStretch",
            SslFieldShapeType::RightPenaltyStretch => "RightPenaltyStretch",
            SslFieldShapeType::LeftFieldLeftPenaltyStretch => {
                "LeftFieldLeftPenaltyStretch"
            }
            SslFieldShapeType::LeftFieldRightPenaltyStretch => {
                "LeftFieldRightPenaltyStretch"
            }
            SslFieldShapeType::RightFieldLeftPenaltyStretch => {
                "RightFieldLeftPenaltyStretch"
            }
            SslFieldShapeType::RightFieldRightPenaltyStretch => {
                "RightFieldRightPenaltyStretch"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Undefined" => Some(Self::Undefined),
            "CenterCircle" => Some(Self::CenterCircle),
            "TopTouchLine" => Some(Self::TopTouchLine),
            "BottomTouchLine" => Some(Self::BottomTouchLine),
            "LeftGoalLine" => Some(Self::LeftGoalLine),
            "RightGoalLine" => Some(Self::RightGoalLine),
            "HalfwayLine" => Some(Self::HalfwayLine),
            "CenterLine" => Some(Self::CenterLine),
            "LeftPenaltyStretch" => Some(Self::LeftPenaltyStretch),
            "RightPenaltyStretch" => Some(Self::RightPenaltyStretch),
            "LeftFieldLeftPenaltyStretch" => Some(Self::LeftFieldLeftPenaltyStretch),
            "LeftFieldRightPenaltyStretch" => Some(Self::LeftFieldRightPenaltyStretch),
            "RightFieldLeftPenaltyStretch" => Some(Self::RightFieldLeftPenaltyStretch),
            "RightFieldRightPenaltyStretch" => Some(Self::RightFieldRightPenaltyStretch),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslWrapperPacket {
    #[prost(message, optional, tag = "1")]
    pub detection: ::core::option::Option<SslDetectionFrame>,
    #[prost(message, optional, tag = "2")]
    pub geometry: ::core::option::Option<SslGeometryData>,
}
