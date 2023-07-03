use crate::data::{FilterData, TrackedRobot};
use crabe_framework::data::world::{RobotMap, World};

use crate::post_filter::PostFilter;

pub struct RobotFilter;

fn insert_tracked<'a, T: Clone + 'a>(
    robot_map: &mut RobotMap<T>,
    tracked: impl Iterator<Item = (&'a u8, &'a TrackedRobot<T>)>,
    positive_change: bool
) {
    robot_map.clear();
    robot_map.extend(
        tracked.map(|(robot_id, tracked_robot)| (robot_id.clone(), tracked_robot.data.clone())),
    );
    if !positive_change{return;}
    for (_i,robot) in robot_map{
        robot.pose.position.x = -robot.pose.position.x;
        robot.pose.position.y = -robot.pose.position.y;
    };
}

impl PostFilter for RobotFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let positive_change = world.data.positive_half == world.team_color;
        insert_tracked(&mut world.enemies_bot, filter_data.enemies.iter(), positive_change);
        insert_tracked(&mut world.allies_bot, filter_data.allies.iter(), positive_change);
    }
}
