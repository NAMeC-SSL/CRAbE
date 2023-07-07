use clap::Args;

/// A struct representing some options that are common to multiple CRAbE crates.
#[derive(Args)]
pub struct CommonConfig {
    /// Whether robots are on the yellow team.
    #[arg(short = 'y', long)]
    pub yellow: bool,
    /// Whether robots are operating in the real world or in simulation.
    #[arg(short, long)]
    pub real: bool,
    /// Specifies whether to activate the game-controller of the Robocup SSL.
    #[arg(long)]
    pub gc: bool,
    #[arg(long)]
    pub tracker: bool,
}