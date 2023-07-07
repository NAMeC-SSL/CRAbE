use chrono::{DateTime, Utc};
use nalgebra::{Point2, Point3, Vector2, Vector3};
use crabe_framework::data::world::{Ball, Pose, Robot, RobotVelocity, TeamColor, World};
use crabe_protocol::protobuf::tracker_vision_packet::TrackedRobot;
use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crate::pre_filter::common::create_date_time;

pub struct AutoRefFilter;

impl AutoRefFilter {
    pub fn new() -> AutoRefFilter {
        AutoRefFilter
    }
    fn map_robot<T: Default>(tracked: &TrackedRobot, timestamp: DateTime<Utc>) -> Robot<T> {
        Robot {
            id: tracked.robot_id.id as u8,
            has_ball: false,
            robot_info: Default::default(),
            pose: Pose { orientation: tracked.orientation.into(), position: Point2::new(tracked.pos.x.into(), tracked.pos.y.into()) },
            velocity: RobotVelocity { linear: tracked.vel.as_ref().map_or(Default::default(), |vel| Vector2::new(vel.x.into(), vel.y.into())), angular: tracked.vel_angular.unwrap_or(0.0).into() },
            acceleration: Default::default(),
            timestamp,
        }
    }
}

impl PostFilter for AutoRefFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        world.allies_bot.clear();
        world.enemies_bot.clear();
        if let Some(packet) = filter_data.tracker.iter().last() {
            for frame in &packet.tracked_frame {
                let timestamp = Default::default(); // TODO
                //let timestamp = create_date_time(frame.timestamp.into());
                for robot in frame.robots.iter() {
                    // TODO: Team color enum
                    let team_color = match robot.robot_id.team_color {
                        1 => {
                            TeamColor::Yellow
                        },
                        _ => {
                            TeamColor::Blue
                        },
                    };

                    if team_color == world.team_color {
                        let robot = Self::map_robot(robot, timestamp);
                        world.allies_bot.insert(robot.id, robot);
                    } else {
                        let robot = Self::map_robot(robot, timestamp);
                        world.enemies_bot.insert(robot.id, robot);
                    }
                }

                let tracked_ball = frame.balls.last();
                let ball = tracked_ball.map(|tracked_ball| {
                    Ball {
                        position: Point3::from_slice(&[tracked_ball.pos.x.into(), tracked_ball.pos.y.into(), tracked_ball.pos.z.into()]),
                        timestamp,
                        velocity: tracked_ball.vel.as_ref().map_or(Default::default(), |vel| Vector3::new(vel.x.into(), vel.y.into(), vel.z.into())),
                        acceleration: Default::default(),
                    }
                });

                world.ball = ball;
            }
        }
    }
}