use crate::speed::SpeedGuard;
use clap::Args;
use crabe_framework::component::{Component, GuardComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;

#[derive(Args)]
pub struct GuardConfig {}

pub trait Guard {
    fn guard(
        &mut self,
        world: &World,
        commands: &mut CommandMap,
        tools_commands: &mut ToolCommands,
    );
}

pub struct GuardPipeline {
    guards: Vec<Box<dyn Guard>>,
}

impl GuardPipeline {
    pub fn with_config(_guard_cfg: GuardConfig, _common_cfg: &CommonConfig) -> Self {
        Self {
            guards: vec![Box::<SpeedGuard>::default()],
        }
    }
}

impl Component for GuardPipeline {
    fn close(self) {}
}

impl GuardComponent for GuardPipeline {
    fn step(
        &mut self,
        world: &World,
        commands: &mut CommandMap,
        tools_commands: &mut ToolCommands,
    ) {
        self.guards
            .iter_mut()
            .for_each(|x| x.guard(world, commands, tools_commands));
    }
}
