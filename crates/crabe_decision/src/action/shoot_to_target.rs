use nalgebra::{distance, Point2};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crate::action::Action;
use crate::action::block_enemy::BlockEnemy;
use crate::action::move_to::MoveTo;
use crate::action::state::State;

#[derive(Clone)]
pub struct ShootToTarget {
    /// The current state of the action.
    state: State,
    /// The target position to shoot towards.
    shoot_target: Point2<f64>
}

impl From<&mut ShootToTarget> for ShootToTarget {
    fn from(other: &mut ShootToTarget) -> ShootToTarget {
        ShootToTarget {
            state: other.state,
            shoot_target: other.shoot_target
        }
    }
}

impl ShootToTarget {
    pub fn new(shoot_target: Point2<f64>) -> Self {
        ShootToTarget {
            state: State::Running,
            shoot_target
        }
    }

    fn look_towards(start: &Point2<f64>, end: &Point2<f64>) -> f64 {
        let y = end.y - start.y;
        let x = end.x - start.x;
        y.atan2(x)
    }
}

impl Action for ShootToTarget {
    fn name(&self) -> String {
        String::from("ShootToTarget")
    }

    fn state(&mut self) -> State {
        self.state
    }

    fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command {
        // if let Some(ball) = &world.ball {
        //     if let Some(&ally) = &world.allies_bot.get(&id) {
        //         let dist_to_ball = distance(&ball.position.xy(), &ally.pose.position);
        //         let mut command = MoveTo::new(
        //             ball.position.xy(),
        //             ShootToTarget::look_towards(&ally.pose.position, &ball.position.xy())
        //         );
        //
        //         if dist_to_ball <= 0.02 {
        //             // shoot
        //
        //             Command {
        //                 forward_velocity: 0.0,
        //                 left_velocity: 0.0,
        //                 angular_velocity: 0.0,
        //                 charge: false,
        //                 kick: None,
        //                 dribbler: 0.0,
        //             }
        //         }
        //     }
        // }
        Command::default()
    }
}