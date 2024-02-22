use clap::{arg, Parser};
use log::LevelFilter;

#[derive(Parser, Debug, Clone)]
pub(crate) struct CliArgs {
    /// Tapo Username (E-Mail)
    #[arg(short = 'u', long = "user")]
    pub user: String,

    /// Hub IP address
    #[arg(short = 'i', long = "hub-ip")]
    pub ip: String,

    /// Device Identifier (if not unique)
    #[arg(short = 's', long = "device", default_value = None)]
    pub device: Option<String>,

    /// Update interval (in seconds)
    #[arg(short = 'n', long = "update-interval", default_value_t = 120)]
    pub interval: u64,

    /// Log level
    #[arg(short = 'l', long = "log-level", default_value_t = LevelFilter::Info)]
    pub log_level: LevelFilter,
}
