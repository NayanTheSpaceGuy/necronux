use color_eyre::{config::HookBuilder, eyre::Result};
use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use log::{debug, info};
use crate::{
    commands::lvl_0::necronux::NecronuxCommand,
    logger::init_logger,
    handlers::init::init_handlers
};

fn init_error_reporter() -> Result<()> {

    #[cfg(debug_assertions)]
    {
        std::env::set_var("RUST_BACKTRACE", "full");
        HookBuilder::default()
            .display_env_section(false)
            .install()?;
    }

    #[cfg(not(debug_assertions))]
    {
        std::env::set_var("RUST_BACKTRACE", "0");
        HookBuilder::default()
            .display_env_section(false)
            .install()?;
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub necronux_command: Option<NecronuxCommand>,

    #[command(flatten)]
    pub verbose: Verbosity<WarnLevel>,
}

pub fn init_cli_controller() -> Result<()> {

    init_error_reporter()?;
    let cli = Cli::parse();
    init_logger(&cli)?;
    info!("Initialized error reporter, cli arguments parser and logger");

    debug!("Parsed CLI arguments: {:?}", cli);

    info!("Initializing handlers");
    init_handlers(&cli)?;

    debug!("Cli controller initialization completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;
    use super::Cli;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
