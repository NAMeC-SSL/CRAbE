use clap::Args;
#[derive(Args)]
pub struct SimulatorConfig {
    #[arg(long)]
    pub simulator_port: Option<u16>,
}
