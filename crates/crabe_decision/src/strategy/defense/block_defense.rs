use std::collections::HashMap;
use std::ops::Add;
use nalgebra::{distance, Isometry2, Point2, Translation, Translation2, Vector2};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Pose, Robot, RobotMap, World};
use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::action::block_enemy::BlockEnemy;
use crate::action::shoot_to_target::ShootToTarget;
use crate::strategy::Strategy;

#[derive(Default)]
pub struct BlockDefense {
    defenders_ids: Vec<u8>,
    defend_dist_mult: f64,
}

impl BlockDefense {
    /// Base initializer, with fixed IDs for defenders
    pub fn new(_: u8) -> Self {
        Self { defenders_ids: vec![2], defend_dist_mult: 1.0 }
    }

    /// In charge of determining which allies should be defending on which enemies
    fn assign_allies_to_enemies<'a>(allies: &'a RobotMap<AllyInfo>, enemies: &'a RobotMap<EnemyInfo>) -> HashMap<u8, &'a Robot<EnemyInfo>> {
        let mut allies_enemies_assignment_map: HashMap<u8, &Robot<EnemyInfo>> = HashMap::new();
        for ally in allies.values() {
            let mut min_dist = 0.0;
            let mut ally_rob_min_id = 0;
            let mut enemy_to_defend: &Robot<EnemyInfo>;
            if let Some(enn) = enemies.get(&0) {
                enemy_to_defend = enn;
            } else {
                println!("no enemies!!11!1!1!11!1!");
                return allies_enemies_assignment_map;
            }

            for enemy in enemies.values() {
                // defend only enemies that we didn't affect yet
                if allies_enemies_assignment_map.values().any(|&assigned_enemy| enemy.id == assigned_enemy.id) {
                    // assign robots that are closer to the enemy
                    let dist= distance(&ally.pose.position, &enemy.pose.position);
                    if dist < min_dist {
                        min_dist = dist;
                        ally_rob_min_id = ally.id;
                        enemy_to_defend = &enemy;
                    }
                }
            }

            allies_enemies_assignment_map.insert(ally.id, &enemy_to_defend);
        }

        allies_enemies_assignment_map
    }

}

impl Strategy for BlockDefense {
    fn step(&mut self, world: &World, tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) -> bool {
        // > empty action_wrapper : TODO
        // -- Translation example in nalgebra
        // let p: Point2<f64> = Point2::new(1.0, 1.0);
        // let v: Vector2<f64> = Vector2::new(1.0, 0.0);
        // let translation = Translation::from(v);
        // let new_p = translation.transform_point(&p);
        // dbg!(new_p);
        // return false;
        // -- new_p gets the value (2.0, 1.0)

        // TODO: yummy, bye bye older actions
        for id in &self.defenders_ids {
            action_wrapper.clean(*id);
        }
        let mut my_allies = world.allies_bot.clone();
        my_allies.retain(|ally_id, _| self.defenders_ids.contains(ally_id));
        let allies_enemies_assignment_map= BlockDefense::assign_allies_to_enemies(&my_allies, &world.enemies_bot);
        // > compute moveto order for each robot
        let mut i: u8= 0;
        if let Some(ball) = &world.ball {
            for (ally_id, &enemy_assigned) in allies_enemies_assignment_map.iter() {
                // let defend_point = BlockDefense::compute_defend_point(enemy_assigned, &ball, self.defend_dist_mult);
                // TODO: look towards ball
                dbg!(enemy_assigned.id);
                // action_wrapper.push(dbg!(*ally_id), MoveTo::new(defend_point, 0.));
                action_wrapper.push(dbg!(*ally_id), ShootToTarget::new(enemy_assigned.id));
                i += 1;
            }
            dbg!(i);
        }
        false
    }
}