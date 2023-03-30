use crate::data::{FilterData, TrackedRobot};
use crabe_framework::data::world::{RobotMap, World};

use crate::post_filter::PostFilter;

pub struct RobotFilter;

fn insert_tracked<'a, T: Clone + 'a>(
    robot_map: &mut RobotMap<T>,
    tracked: impl Iterator<Item = (&'a u8, &'a TrackedRobot<T>)>,
) {
    robot_map.clear();
    robot_map.extend(
        tracked.map(|(robot_id, tracked_robot)| (robot_id.clone(), tracked_robot.data.clone())),
    )
}

impl PostFilter for RobotFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        insert_tracked(&mut world.enemies_bot, filter_data.enemies.iter());
        insert_tracked(&mut world.allies_bot, filter_data.allies.iter());
    }
}
