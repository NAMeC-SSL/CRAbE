use crabe_framework::component::{Component, OutputComponent};
use crabe_framework::config::CommonConfig;

use crabe_framework::data::output::{CommandMap, FeedbackMap};
use crabe_framework::data::tool::ToolCommands;

use crate::league::real::{Real, RealConfig};
use crate::league::simulator::config::SimulatorConfig;
use crate::league::simulator::task::Simulator;
use clap::Args;

#[derive(Args)]
pub struct OutputConfig {
    #[command(flatten)]
    #[command(next_help_heading = "Real")]
    pub real_cfg: RealConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Simulation")]
    pub simulator_cfg: SimulatorConfig,
}

pub trait CommandSenderTask {
    fn step(&mut self, commands: CommandMap) -> FeedbackMap;
    fn close(&mut self);
}

pub struct OutputPipeline {
    command_task: Box<dyn CommandSenderTask>,
}

impl OutputPipeline {
    pub fn with_config(output_cfg: OutputConfig, common_cfg: &CommonConfig) -> OutputPipeline {
        let command_task: Box<dyn CommandSenderTask> = if common_cfg.real {
            Box::new(Real::with_config(output_cfg.real_cfg))
        } else {
            Box::new(Simulator::with_config(output_cfg.simulator_cfg, common_cfg))
        };

        OutputPipeline { command_task }
    }
}

impl Component for OutputPipeline {
    fn close(mut self) {
        self.command_task.close()
    }
}

impl OutputComponent for OutputPipeline {
    fn step(&mut self, commands: CommandMap, _tool_commands: ToolCommands) -> FeedbackMap {
        self.command_task.step(commands)
    }
}
