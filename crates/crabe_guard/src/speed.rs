use std::cmp::max;
use std::ops::Div;
use crate::constant::{MAX_ANGULAR, MAX_LINEAR};
use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;

pub struct SpeedGuard {
    max_linear: f32,
    max_angular: f32,
}

impl SpeedGuard {
    pub fn new(max_linear: f32, max_angular: f32) -> Self {
        Self {
            max_linear,
            max_angular,
        }
    }
}

impl Default for SpeedGuard {
    fn default() -> Self {
        Self {
            max_linear: MAX_LINEAR,
            max_angular: MAX_ANGULAR,
        }
    }
}

impl Guard for SpeedGuard {
    fn guard(
        &mut self,
        _world: &World,
        commands: &mut CommandMap,
        _tool_commands: &mut ToolCommands,
    ) {
        commands.iter_mut().for_each(|(_id, command)| {
            // Determine speed factors of each vector
            let fact_vx: f32 =  command.forward_velocity / self.max_linear;
            let fact_vy: f32 =  command.left_velocity / self.max_linear;
            let fact_vt: f32 =  command.angular_velocity / self.max_angular;

            let factor_max: f32 = fact_vx.abs().max(fact_vy.abs()).max(fact_vt.abs());
            println!("{}", factor_max);

            if factor_max.abs() > 1.0 {
                // Normalize all speeds by the maximum factor computed
                command.forward_velocity = command.forward_velocity / factor_max;
                command.left_velocity = command.left_velocity / factor_max;
                command.angular_velocity = command.angular_velocity / factor_max;
            }

            // // Clamp values to their relative threshold
            // command.forward_velocity = command
            //     .forward_velocity
            //     .clamp(-self.max_linear, self.max_linear);
            // command.left_velocity = command
            //     .left_velocity
            //     .clamp(-self.max_linear, self.max_linear);
            // command.angular_velocity = command
            //     .angular_velocity
            //     .clamp(-self.max_angular, self.max_angular);
        });
    }
}
