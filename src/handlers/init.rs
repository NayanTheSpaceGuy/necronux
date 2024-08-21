use clap::CommandFactory;
use log::{debug, info};
use crate::controller::Cli;
use crate::commands::lvl_0::necronux::NecronuxCommand;
use crate::handlers::{
    global_args::handle_global_args,
    infra::handle_infra
};

pub fn init_handlers(cli: &Cli) {

    debug!("Initializing global arguments handler");
    handle_global_args(&cli.global_args);

    match &cli.necronux_command {
        Some(NecronuxCommand::Infra(infra_command)) => handle_infra(&infra_command),
        Some(NecronuxCommand::System(_)) => {
            println!("System command not yet implemented");
        },
        Some(NecronuxCommand::App(_)) => {
            println!("App command not yet implemented");
        },
        None => {
            info!("No subcommand was provided");
            info!("Displaying help message");
            Cli::command().print_help().unwrap();
        }
    }
}
