use std::collections::HashMap;
use nalgebra::{distance, Point2, Vector, Vector1, Vector2};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, EnemyInfo, Robot, World};
use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;

#[derive(Default)]
pub struct BlockDefense {}

impl BlockDefense {
    pub fn new(_: u8) -> Self {
        Self {}
    }
}

impl Strategy for BlockDefense {
    fn step(&mut self, world: &World, tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) -> bool {
        // > empty action_wrapper : TODO
        let allies_enemies_assignment_map: HashMap<u8, Robot<EnemyInfo>> = self.assign_allies_to_enemies(world);

        // > compute moveto order for each robot
        for rob_id in allies_enemies_assignment_map.keys() {
            if let Some(ally) = world.allies_bot.get(rob_id) {
                let enn = allies_enemies_assignment_map.get(rob_id);
                if let Some(enn) = enn {
                    let vec_before_enn=
                        Vector2::new(
                            ally.pose.position.x - enn.pose.position.x,
                            ally.pose.position.y - enn.pose.position.y,
                        ).normalize()
                    ;
                    action_wrapper.push(*rob_id, MoveTo::new(Point2::from(vec_before_enn), 0.));
                }
            }
        }

        // > push the actions in action_wrapper
        false
    }
}

/// In charge of determining which allies should be defending on which enemies
fn assign_allies_to_enemies(world: &World) -> HashMap<u8, &Robot<EnemyInfo>> {
    let mut allies_enemies_assignment_map: HashMap<u8, &Robot<EnemyInfo>> = HashMap::new();
    let allies = &world.allies_bot;
    for enn in world.enemies_bot.values() {
        // Get min distance possible of robot
        let mut min_dist: f64 = 0.0;
        let mut ally_rob_min_id = 0;
        for ally in allies.values() {
            if !allies_enemies_assignment_map.contains_key(&ally.id) {
                let dist= distance(&ally.pose.position, &enn.pose.position);
                if dist < min_dist {
                    min_dist = dist;
                    ally_rob_min_id = ally.id;
                }
            }
        }
        allies_enemies_assignment_map.insert(ally_rob_min_id, enn);
    }

    return allies_enemies_assignment_map;
}