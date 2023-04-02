use std::time::{Duration, Instant};
use nalgebra::{distance, Point2};
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crate::action::Action;
use crate::action::block_enemy::BlockEnemy;
use crate::action::move_to::{How, MoveTo};
use crate::action::state::State;

#[derive(Clone)]
pub struct ShootToTarget {
    /// The current state of the action.
    state: State,
    /// The target position to shoot towards.
    shoot_target: Point2<f64>,
    /// Last time the command has pushed a kick
    kick_time: Instant

}

impl From<&mut ShootToTarget> for ShootToTarget {
    fn from(other: &mut ShootToTarget) -> ShootToTarget {
        ShootToTarget {
            state: other.state,
            shoot_target: other.shoot_target,
            kick_time: other.kick_time
        }
    }
}

impl ShootToTarget {
    pub fn new(shoot_target: Point2<f64>) -> Self {
        ShootToTarget {
            state: State::Running,
            shoot_target,
            kick_time: Instant::now()
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
        if let Some(ball) = &world.ball {
            if let Some(ally) = &world.allies_bot.get(&id) {
                let dist_to_ball = distance(&ball.position.xy(), &ally.pose.position);
                let mut shoot_angle = ShootToTarget::look_towards(&ally.pose.position, &ball.position.xy());

                if dist_to_ball <= 0.02 {
                    // we should already be dribbling
                    // aim towards target
                    shoot_angle = ShootToTarget::look_towards(&ball.position.xy(), &self.shoot_target);
                }

                let mut base_command = MoveTo::new(
                    None,
                    ball.position.xy(),
                    shoot_angle,
                    How::Accurate
                );
                let mut command= base_command.compute_order(id, world, tools);


                if dist_to_ball <= 0.02 {
                    // shoot if time elapsed is okay
                    if self.kick_time.elapsed() >= Duration::from_secs(2) {
                        command.kick = Option::from(Kick::StraightKick { power: 0.9 });
                        self.kick_time = Instant::now();
                    }
                } else if dist_to_ball <= 0.1 {
                    // dribble
                    command.dribbler = 1.0;
                } else {
                    base_command.update_target(ball.position.xy());
                    command = base_command.compute_order(id, world, tools);
                }
                return command;
            }
        }
        Command::default()
    }
}