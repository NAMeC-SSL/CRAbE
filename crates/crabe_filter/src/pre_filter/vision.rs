use crate::data::FilterData;
use crate::PreFilter;

use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;

mod detection {
    use crate::data::{FilterData, FrameInfo};
    use chrono::{DateTime, LocalResult, TimeZone, Utc};
    use crabe_framework::data::world::TeamColor;
    use crabe_protocol::protobuf::vision_packet::SslDetectionFrame;
    use log::error;

    mod robot {
        use crate::data::{camera::CamRobot, FrameInfo, TrackedRobot, TrackedRobotMap};
        use crabe_framework::constant::MAX_ID_ROBOTS;
        use crabe_framework::data::world::{AllyInfo, EnemyInfo, Robot, TeamColor};
        use crabe_protocol::protobuf::vision_packet::SslDetectionRobot;
        use log::warn;
        use nalgebra::Point2;
        use ringbuffer::RingBufferWrite;

        pub struct RobotDetectionInfo<'a> {
            pub detected_blue: &'a [SslDetectionRobot],
            pub detected_yellow: &'a [SslDetectionRobot],
            pub tracked_allies: &'a mut TrackedRobotMap<AllyInfo>,
            pub tracked_enemies: &'a mut TrackedRobotMap<EnemyInfo>,
        }

        fn track_robots<T: Default>(
            robots: &mut TrackedRobotMap<T>,
            cam_robots: impl Iterator<Item = CamRobot>,
        ) {
            cam_robots.for_each(|r| {
                let robot = robots.entry(r.id).or_insert_with(|| TrackedRobot {
                    data: Robot {
                        id: r.id,
                        ..Default::default()
                    },
                    ..Default::default()
                });
                robot.last_update = r.frame_info.t_capture;
                robot.packets.push(r);
            })
        }

        pub fn detect_robots(
            detection: &mut RobotDetectionInfo,
            frame: &FrameInfo,
            team_color: &TeamColor,
        ) {
            let map_packet = |r: &SslDetectionRobot| {
                r.robot_id
                    .map(|id| {
                        if id > MAX_ID_ROBOTS as u32 {
                            warn!("invalid id");
                            return None;
                        } else {
                            Some(CamRobot {
                                id: id as u8,
                                frame_info: frame.clone(),
                                position: Point2::new(r.x as f64 / 1000.0, r.y as f64 / 1000.0),
                                orientation: r.orientation.unwrap_or(0.0) as f64,
                                confidence: r.confidence as f64,
                            })
                        }
                    })
                    .flatten()
            };

            let yellow = detection.detected_yellow.iter().filter_map(map_packet);
            let blue = detection.detected_blue.iter().filter_map(map_packet);

            let allies;
            let enemies;

            match team_color {
                TeamColor::Yellow => {
                    allies = yellow;
                    enemies = blue;
                }

                _ => {
                    allies = blue;
                    enemies = yellow;
                }
            }

            track_robots(detection.tracked_allies, allies);
            track_robots(detection.tracked_enemies, enemies);
        }
    }

    mod ball {
        use crate::data::{camera::CamBall, FrameInfo, TrackedBall};
        use crabe_protocol::protobuf::vision_packet::SslDetectionBall;
        use nalgebra::Point3;

        pub struct BallDetectionInfo<'a> {
            pub detected: &'a [SslDetectionBall],
            pub tracked: &'a mut TrackedBall,
        }

        pub fn detect_balls(detection: &mut BallDetectionInfo, frame: &FrameInfo) {
            let ball_packets = detection.detected.iter().map(|b| CamBall {
                frame_info: frame.clone(),
                position: Point3::new(
                    b.x as f64 / 1000.0,
                    b.y as f64 / 1000.0,
                    b.z.unwrap_or(0.0) as f64 / 1000.0,
                ),
                confidence: b.confidence as f64,
            });

            detection.tracked.packets.extend(ball_packets);
        }
    }

    fn create_date_time(t_capture: f64) -> DateTime<Utc> {
        match Utc.timestamp_opt((t_capture) as i64, 0) {
            LocalResult::Single(dt) => dt,
            LocalResult::None => {
                let now_utc = Utc::now();
                error!("Invalid timestamp, using current time: {}", now_utc);
                now_utc
            }
            LocalResult::Ambiguous(dt_min, dt_max) => {
                let dt_midpoint = dt_min + (dt_max - dt_min) / 2;
                error!("Ambiguous timestamp resolved to midpoint: {}", dt_midpoint);
                dt_midpoint
            }
        }
    }

    pub fn handle_detection(
        detection: &SslDetectionFrame,
        filter_data: &mut FilterData,
        team_color: &TeamColor,
    ) {
        let frame_info = FrameInfo {
            camera_id: detection.camera_id,
            frame_number: detection.frame_number,
            t_capture: create_date_time(Utc::now().timestamp() as f64),
        };

        let mut robot_detection_info = robot::RobotDetectionInfo {
            detected_yellow: &detection.robots_yellow,
            detected_blue: &detection.robots_blue,
            tracked_allies: &mut filter_data.allies,
            tracked_enemies: &mut filter_data.enemies,
        };

        robot::detect_robots(&mut robot_detection_info, &frame_info, team_color);

        let mut ball_detection_info = ball::BallDetectionInfo {
            detected: &detection.balls,
            tracked: &mut filter_data.ball,
        };

        ball::detect_balls(&mut ball_detection_info, &frame_info)
    }
}

mod geometry {
    use crate::data::camera::{CamFieldArc, CamFieldLine};
    use crate::data::{camera::CamGeometry, FilterData};
    use crabe_math::shape::Arc;
    use crabe_math::shape::Line;
    use crabe_protocol::protobuf::vision_packet::SslGeometryData;
    use nalgebra::Point2;
    use std::collections::HashMap;

    pub fn handle_geometry(geometry: &SslGeometryData, filter_data: &mut FilterData) {
        let mut cam_geometry = CamGeometry {
            field_length: geometry.field.field_length as f64 / 1000.0,
            field_width: geometry.field.field_width as f64 / 1000.0,
            goal_width: geometry.field.goal_width as f64 / 1000.0,
            goal_depth: geometry.field.goal_depth as f64 / 1000.0,
            boundary_width: geometry.field.boundary_width as f64 / 1000.0,
            field_lines: HashMap::new(),
            field_arcs: HashMap::new(),
            penalty_area_depth: geometry.field.penalty_area_depth.map(|v| v as f64 / 1000.0),
            penalty_area_width: geometry.field.penalty_area_width.map(|v| v as f64 / 1000.0),
            center_circle_radius: geometry
                .field
                .center_circle_radius
                .map(|v| v as f64 / 1000.0),
            line_thickness: geometry.field.line_thickness.map(|v| v as f64 / 1000.0),
            goal_center_to_penalty_mark: geometry
                .field
                .goal_center_to_penalty_mark
                .map(|v| v as f64 / 1000.0),
            goal_height: geometry.field.goal_height.map(|v| v as f64 / 1000.0),
            ball_radius: geometry.field.ball_radius.map(|v| v as f64 / 1000.0),
            max_robot_radius: geometry.field.max_robot_radius.map(|v| v as f64 / 1000.0),
        };

        geometry.field.field_lines.iter().for_each(|line| {
            cam_geometry.field_lines.insert(
                line.name.clone(),
                CamFieldLine {
                    thickness: line.thickness as f64 / 1000.0,
                    line: Line {
                        start: Point2::new(line.p1.x as f64 / 1000.0, line.p1.y as f64 / 1000.0),
                        end: Point2::new(line.p2.x as f64 / 1000.0, line.p2.y as f64 / 1000.0),
                    },
                },
            );
        });

        geometry.field.field_arcs.iter().for_each(|arc| {
            cam_geometry.field_arcs.insert(
                arc.name.clone(),
                CamFieldArc {
                    thickness: arc.thickness as f64 / 1000.0,
                    arc: Arc {
                        center: Point2::new(
                            arc.center.x as f64 / 1000.0,
                            arc.center.y as f64 / 1000.0,
                        ),
                        radius: arc.radius as f64 / 1000.0,
                        start: arc.a1 as f64,
                        end: arc.a2 as f64,
                    },
                },
            );
        });

        filter_data.geometry = cam_geometry;
    }
}

pub struct VisionFilter;

impl VisionFilter {
    pub fn new() -> VisionFilter {
        VisionFilter
    }
}

impl PreFilter for VisionFilter {
    fn step(
        &mut self,
        inbound_data: &InboundData,
        team_color: &TeamColor,
        filter_data: &mut FilterData,
    ) {
        inbound_data.vision_packet.iter().for_each(|packet| {
            if let Some(detection) = packet.detection.as_ref() {
                detection::handle_detection(detection, filter_data, team_color);
            }

            if let Some(geometry) = packet.geometry.as_ref() {
                geometry::handle_geometry(geometry, filter_data);
            }
        });
    }
}
