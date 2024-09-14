use crate::{
    commands::lvl_0::necronux::NecronuxCommand, controller::Cli,
    handlers::infra::infra_root::handle_infra,
};
use clap::CommandFactory;
use color_eyre::eyre::Result;
use log::{debug, info};

pub fn init_handlers(cli: &Cli) -> Result<()> {
    match &cli.necronux_command {
        Some(NecronuxCommand::Infra(infra_command)) => {
            info!("'infra' command was provided");
            debug!("Handling infra command");
            handle_infra(infra_command)?;
            debug!("Finished handling infra command");
        }
        Some(NecronuxCommand::System(_)) => {
            info!("'system' command was provided");
            info!("'system' command not yet implemented");
        }
        Some(NecronuxCommand::App(_)) => {
            info!("'app' command was provided");
            info!("'app' command not yet implemented");
        }
        None => {
            info!("No subcommand was provided");
            info!("Displaying help message");
            Cli::command().print_help()?;
        }
    }

    Ok(())
}
