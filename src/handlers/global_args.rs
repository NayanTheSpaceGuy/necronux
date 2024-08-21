use log::{debug, info};
use crate::{
    commands::lvl_0::global_args::GlobalArgs,
    logger::LogLevel
};

pub fn handle_global_args(args: &GlobalArgs) {

    match args.log_level {
        Some(LogLevel::Off) => println!("Log level set to off"),
        Some(LogLevel::Error) => println!("Log level set to error"),
        Some(LogLevel::Warn) => println!("Log level set to warn"),
        Some(LogLevel::Info) => info!("Log level set to info"),
        #[cfg(debug_assertions)]
        Some(LogLevel::Debug) => info!("Log level set to debug"),
        #[cfg(debug_assertions)]
        Some(LogLevel::Trace) => info!("Log level set to trace"),
        None => debug!("No specific log level set with the log-level flag"),
    }

    match args.quiet {
        true => println!("Quiet mode enabled: all logs will be suppressed except errors"),
        false => debug!("Quiet mode not enabled"),
    }

    match args.verbose {
        true => info!("Verbose mode enabled: additional info logs will be shown"),
        false => debug!("Verbose mode not enabled"),
    }

    #[cfg(debug_assertions)]
    match args.debug {
        true => info!("Debug mode enabled"),
        false => debug!("Debug mode not enabled"),
    }

    #[cfg(debug_assertions)]
    match args.trace {
        true => info!("Trace mode enabled"),
        false => debug!("Trace mode not enabled"),
    }
}
