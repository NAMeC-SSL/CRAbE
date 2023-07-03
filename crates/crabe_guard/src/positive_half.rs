use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::{World};

pub struct HalfGuard {}

impl HalfGuard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for HalfGuard {
    fn default() -> Self {
        Self {}
    }
}

impl Guard for HalfGuard {
    fn guard(
        &mut self,
        world: &World,
        commands: &mut CommandMap,
        _tool_commands: &mut ToolCommands,
    ) {
        if world.data.positive_half != world.team_color{return;}
        commands.iter_mut().for_each(|(_id, command)| {
            command.forward_velocity = -command
                .forward_velocity;
            command.left_velocity = -command
                .left_velocity;
            command.angular_velocity = -command
                .angular_velocity
        });
    }
}
