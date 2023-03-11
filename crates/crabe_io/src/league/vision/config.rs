use clap::Args;

/// Represents the configuration settings for SSL-Vision or the Simulator vision
/// module.
#[derive(Args)]
pub struct VisionConfig {
    #[arg(long, default_value = "224.5.23.2")]
    pub vision_ip: String,
    pub vision_port: Option<u16>,
}
