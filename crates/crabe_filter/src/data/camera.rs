use crate::data::FrameInfo;
use crabe_math::shape::Arc;
use crabe_math::shape::Line;
use nalgebra::{Point2, Point3};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CamBall {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub frame_info: FrameInfo,
    pub confidence: f64,
}

#[derive(Debug)]
pub struct CamRobot {
    pub id: u8,
    pub frame_info: FrameInfo,
    pub position: Point2<f64>,
    pub orientation: f64,
    pub confidence: f64,
}

#[derive(Debug)]
pub struct CamFieldLine {
    pub thickness: f64,
    pub line: Line,
}

#[derive(Debug)]
pub struct CamFieldArc {
    pub thickness: f64,
    pub arc: Arc,
}

#[derive(Debug, Default)]
pub struct CamGeometry {
    pub field_length: f64,
    pub field_width: f64,
    pub goal_width: f64,
    pub goal_depth: f64,
    pub boundary_width: f64,
    pub field_lines: HashMap<String, CamFieldLine>,
    pub field_arcs: HashMap<String, CamFieldArc>, // pub last_update: Instant,
    pub penalty_area_depth: Option<f64>,
    pub penalty_area_width: Option<f64>,
    pub center_circle_radius: Option<f64>,
    pub line_thickness: Option<f64>,
    pub goal_center_to_penalty_mark: Option<f64>,
    pub goal_height: Option<f64>,
    pub ball_radius: Option<f64>,
    pub max_robot_radius: Option<f64>,
}
