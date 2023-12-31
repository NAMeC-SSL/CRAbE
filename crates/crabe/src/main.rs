use clap::Parser;
use crabe_decision::pipeline::{DecisionConfig, DecisionPipeline};
use crabe_filter::{FilterConfig, FilterPipeline};
use crabe_framework::component::{
    Component, DecisionComponent, FilterComponent, GuardComponent, InputComponent, OutputComponent,
    ToolComponent,
};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::FeedbackMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;
use crabe_guard::pipeline::{GuardConfig, GuardPipeline};
use crabe_io::pipeline::input::{InputConfig, InputPipeline};
use crabe_io::pipeline::output::{OutputConfig, OutputPipeline};
use crabe_io::tool::ToolConfig;
use crabe_io::tool::ToolServer;
use env_logger::Env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about = "Central AI of NAMeC - runs by default in sim", long_about)]
pub struct Cli {
    #[command(flatten)]
    #[command(next_help_heading = "Common")]
    pub common: CommonConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Input")]
    pub input_config: InputConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Filter")]
    pub filter_config: FilterConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Decision")]
    pub decision_config: DecisionConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Tool")]
    pub tool_config: ToolConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Guard")]
    pub guard_config: GuardConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Output")]
    pub output_config: OutputConfig,
}

fn main() {
    let cli = Cli::parse();
    let env = Env::default()
        .filter_or("CRABE_LOG_LEVEL", "info")
        .write_style_or("CRABE_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let mut world = World::with_config(&cli.common);
    let mut input_component = InputPipeline::with_config(cli.input_config, &cli.common);
    let mut filter_component = FilterPipeline::with_config(cli.filter_config, &cli.common);
    let mut decision_component = DecisionPipeline::with_config(
        cli.decision_config,
        &cli.common,
    );
    let mut tool_component = ToolServer::with_config(cli.tool_config, &cli.common);
    let mut guard_component = GuardPipeline::with_config(cli.guard_config, &cli.common);
    let mut output_component = OutputPipeline::with_config(cli.output_config, &cli.common);

    let refresh_rate = Duration::from_millis(16);
    let mut feedback: FeedbackMap = Default::default();

    let running = {
        let running = Arc::new(AtomicBool::new(true));
        let running_ctrlc = Arc::clone(&running);
        ctrlc::set_handler(move || {
            running_ctrlc.store(false, Ordering::Relaxed);
        })
            .expect("Failed to set Ctrl-C handler");
        running
    };

    while running.load(Ordering::SeqCst) {
        let receive_data = input_component.step(&mut feedback);
        filter_component.step(receive_data, &mut world);
        dbg!(&world.allies_bot);

        let (mut command_map, mut tool_data) = decision_component.step(&world);
        tool_component
            .step(&world, &mut tool_data, &mut command_map);
        guard_component
            .step(&world, &mut command_map, &mut ToolCommands);
        feedback = output_component.step(command_map, ToolCommands);
        thread::sleep(refresh_rate);
    }
}
