use std::env;
use color_eyre::eyre::Result;
use clap::ValueEnum;
use env_logger::{Builder, Env};
use log::{debug, LevelFilter};
use crate::commands::lvl_0::global_args::GlobalArgs;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    #[cfg(debug_assertions)]
    Debug,
    #[cfg(debug_assertions)]
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            #[cfg(debug_assertions)]
            LogLevel::Debug => LevelFilter::Debug,
            #[cfg(debug_assertions)]
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

pub fn init_logger(args: &GlobalArgs) -> Result<()> {

    let mut builder = Builder::new();

    // Set default log level to warn
    builder.filter_level(LevelFilter::Warn);

    let env = Env::new()
        .filter("NECRONUX_LOG_LEVEL")
        .write_style("NECRONUX_LOG_STYLE");

    let env_log_level = env::var("NECRONUX_LOG_LEVEL").ok();

    if let Some(log_level_arg) = args.log_level {
        builder.filter_level(LevelFilter::from(log_level_arg));
    } else {
        #[cfg(debug_assertions)]
        {
            if args.trace {
                builder.filter_level(LevelFilter::Trace);
            } else if args.debug {
                builder.filter_level(LevelFilter::Debug);
            } else if args.verbose {
                builder.filter_level(LevelFilter::Info);
            } else if args.quiet {
                builder.filter_level(LevelFilter::Error);
            } else {
                // Apply environment variable log level
                builder.parse_env(env);
                // Override with appropriate environment variable log level
                match env_log_level.as_deref() {
                    Some("trace") => builder.filter_level(LevelFilter::Trace),
                    Some("debug") => builder.filter_level(LevelFilter::Debug),
                    Some("info") => builder.filter_level(LevelFilter::Info),
                    Some("warn") => builder.filter_level(LevelFilter::Warn),
                    Some("error") => builder.filter_level(LevelFilter::Error),
                    Some("off") => builder.filter_level(LevelFilter::Off),
                    _ => builder.filter_level(LevelFilter::Warn), // Default if invalid or missing
                };
            }

        }
        #[cfg(not(debug_assertions))]
        {
            if args.verbose {
                builder.filter_level(LevelFilter::Info);
            } else if args.quiet {
                builder.filter_level(LevelFilter::Error);
            } else {
                // Apply environment variable log level
                builder.parse_env(env);
                // Override with appropriate environment variable log level
                match env_log_level.as_deref() {
                    Some("info") => builder.filter_level(LevelFilter::Info),
                    Some("error") => builder.filter_level(LevelFilter::Error),
                    Some("off") => builder.filter_level(LevelFilter::Off),
                    _ => builder.filter_level(LevelFilter::Warn), // Default if invalid or missing
                };
            };
        }
    }

    builder
        .format_target(false)
        .format_timestamp_millis()
        .init();
    debug!("Initialized env_logger builder");

    Ok(())
}
