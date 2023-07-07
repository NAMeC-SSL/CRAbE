use crabe_framework::data::world::{Ball, Robot, RobotMap, World};
use crate::data::{FilterData, TrackedBall, TrackedRobotMap};
use crate::FieldSide;
use crate::filter::Filter;
use crate::post_filter::PostFilter;

pub struct FieldSideFilter {
    field_side: FieldSide,
}

impl FieldSideFilter {
    pub fn new(field_side: FieldSide) -> FieldSideFilter {
        FieldSideFilter { field_side }
    }

    fn filter_robots_by_side<T>(tracked_robots: &mut RobotMap<T>, field_side: &FieldSide) {
        tracked_robots.retain(|_id, robot| {
            match field_side {
                FieldSide::Positive => robot.pose.position.x.is_sign_positive(),
                FieldSide::Negative => robot.pose.position.x.is_sign_negative()
            }
        });
    }

    fn filter_ball_by_side(tracked_ball: &mut Option<Ball>, field_side: &FieldSide) {
        *tracked_ball = tracked_ball.take().filter(|ball| {
            match field_side {
                FieldSide::Positive => ball.position.x.is_sign_positive(),
                FieldSide::Negative => ball.position.x.is_sign_negative()
            }
        })
    }
}

impl PostFilter for FieldSideFilter {
    fn step(&mut self, _filter_data: &FilterData, world: &mut World) {
        Self::filter_robots_by_side(&mut world.allies_bot, &self.field_side);
        Self::filter_robots_by_side(&mut world.enemies_bot, &self.field_side);
        Self::filter_ball_by_side(&mut world.ball, &self.field_side);
    }
}