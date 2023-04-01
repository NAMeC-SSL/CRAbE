use crate::constant::{MAX_ANGULAR, MAX_LINEAR};
use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;

pub struct FlipGuard {
    max_linear: f32,
    max_angular: f32,
}

impl FlipGuard {
    pub fn new(max_linear: f32, max_angular: f32) -> Self {
        Self {
            max_linear,
            max_angular,
        }
    }
}

impl Default for FlipGuard {
    fn default() -> Self {
        Self {
            max_linear: MAX_LINEAR,
            max_angular: MAX_ANGULAR,
        }
    }
}

impl Guard for FlipGuard {
    fn guard(
        &mut self,
        world: &World,
        commands: &mut CommandMap,
        _tool_commands: &mut ToolCommands,
    ) {
        if world.team_color == world.data.positive_half {
            commands.iter_mut().for_each(|(_id, command)| {
                command.forward_velocity = -command.forward_velocity;
                command.left_velocity = -command.left_velocity;
                command.angular_velocity = (command
                    .angular_velocity + std::f32::consts::PI).rem_euclid(2.0 * std::f32::consts::PI)
            });
        }
    }
}
