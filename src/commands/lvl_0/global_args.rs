use clap::Args;
use crate::logger::LogLevel;

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct GlobalArgs {

    #[arg(long, global = true)]
    // Decrease the logging verbosity level
    pub quiet: bool,

    #[arg(short, long, global = true)]
    // Increase the logging verbosity level
    pub verbose: bool,

    #[cfg(debug_assertions)]
    #[arg(long, global = true)]
    // Set the logging verbosity level to 'debug'
    pub debug: bool,

    #[cfg(debug_assertions)]
    #[arg(long, global = true)]
    // Set the logging verbosity level to 'trace'
    pub trace: bool,

    #[arg(long, value_enum, global = true)]
    pub log_level: Option<LogLevel>,
}
