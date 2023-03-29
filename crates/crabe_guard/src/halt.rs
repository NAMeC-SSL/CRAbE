use crabe_framework::constant::MAX_ID_ROBOTS;
use crabe_framework::data::output::{Command, CommandMap};
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;
use crate::pipeline::Guard;

pub struct HaltGuard;

impl HaltGuard {
    pub fn new() -> Self {
        Self
    }
}

impl Guard for HaltGuard {
    fn guard(&mut self, world: &World, commands: &mut CommandMap, _tool_commands: &mut ToolCommands) {
        if world.vision_timeout {
            let halt_commands = (0..MAX_ID_ROBOTS).map(|id| (id as u8, Command::default()));
            commands.extend(halt_commands);
        }
    }
}