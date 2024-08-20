use crate::{
    commands::lvl_0::global_args::GlobalArgs,
    logger::LogLevel
};

pub fn handle_global_args(args: &GlobalArgs) {
    match args.log_level {
        Some(LogLevel::Off) => println!("Log level set to off"),
        Some(LogLevel::Error) => println!("Log level set to error"),
        Some(LogLevel::Warn) => println!("Log level set to warn"),
        Some(LogLevel::Info) => println!("Log level set to info"),
        #[cfg(debug_assertions)]
        Some(LogLevel::Debug) => println!("Log level set to debug"),
        #[cfg(debug_assertions)]
        Some(LogLevel::Trace) => println!("Log level set to trace"),
        None => ()
    }

    match args.quiet {
        true => println!("Log level set to error"),
        false => (),
    }

    match args.verbose {
        true => println!("Log level set to info"),
        false => ()
    }
}
