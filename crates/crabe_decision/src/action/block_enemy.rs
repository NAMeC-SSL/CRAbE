use nalgebra::{Point2, Translation2, Vector2};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{Ball, EnemyInfo, Robot, World};
use crate::action::Action;
use crate::action::move_to::MoveTo;
use crate::action::state::State;

/// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
#[derive(Clone)]
pub struct BlockEnemy {
    /// The current state of the action.
    state: State,
    enemy_id: u8
}

impl From<&mut BlockEnemy> for BlockEnemy {
    fn from(other: &mut BlockEnemy) -> BlockEnemy {
        BlockEnemy {
            state: other.state,
            enemy_id: other.enemy_id
        }
    }
}

impl BlockEnemy {
    pub fn new(enemy_id: u8) -> Self {
        Self {
            enemy_id,
            state: State::Running //TODO: proper state management for block enemy
        }
    }

    fn compute_defend_point(enemy: &Robot<EnemyInfo>, ball: &Ball, defend_dist_mult: f64) -> Point2<f64> {
        let vec_before_enn =
            Vector2::new(
                ball.position.x - enemy.pose.position.x,
                ball.position.y - enemy.pose.position.y,
            ) * defend_dist_mult
        ;

        // Using the vector from enemy robot to the ball, we create a Translation
        // object that will allow to to translate a point using the given vector
        let translation = Translation2::from(vec_before_enn);
        // Using this translation object, we can compute the new point
        let defend_point = translation.transform_point(&enemy.pose.position);
        dbg!(defend_point)
    }

    fn look_towards(start: &Point2<f64>, end: &Point2<f64>) -> f64 {
        let y = end.y - start.y;
        let x = end.x - start.x;
        y.atan2(x)
    }
}

impl Action for BlockEnemy {
    fn name(&self) -> String { String::from("BlockEnemy") }

    fn state(&mut self) -> State { self.state }

    fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command {
        let enemy_info_opt = world.enemies_bot.get(&self.enemy_id);
        let ball_opt = &world.ball;
        if let Some(enemy_info) = enemy_info_opt {
            if let Some(ball) = ball_opt {
                let target = BlockEnemy::compute_defend_point(enemy_info, ball, 0.75);
                let orientation = BlockEnemy::look_towards(&target, &ball.position.xy());
                let mut move_to = MoveTo::new(
                    target,
                    orientation
                );
                return move_to.compute_order(id, world, tools);
            }
        }
        Command::default()
    }
}