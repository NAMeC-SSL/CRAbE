use clap::Args;

/// Represents the configuration settings for SSL-Vision or the Simulator vision module.
#[derive(Args)]
pub struct VisionConfig {
    #[arg(long, default_value = "224.5.23.2")]
    pub vision_ip: String,

    #[arg(long, default_value_t = 10020)]
    pub vision_port: u32,
}
