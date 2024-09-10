use clap::CommandFactory;
use color_eyre::eyre::{Ok, Result};
use log::{debug, info};
use crate::{
    commands::lvl_1::infra::{InfraCommand, InfraSubCommands},
    controller::Cli,
    handlers::infra::infra_run::handle_infra_run
};

pub fn handle_infra(command: &InfraCommand) -> Result<()> {

    debug!("Handling infra command subcommands");
    let command_handled = handle_infra_command(command)?;
    if !command_handled {

        debug!("No infra subcommand or valid argument provided, displaying help message");
        let mut cli = Cli::command();
        if let Some(infra_command) = cli.find_subcommand_mut("infra") {
            infra_command.print_help()?;
        }
    }

    Ok(())
}

pub fn handle_infra_command(infra_command: &InfraCommand) -> Result<bool> {

    if let Some(subcommand) = &infra_command.infra_subcommand {
        match subcommand {
            InfraSubCommands::Run(args) => {
                info!("'infra run' command was provided");
                debug!("Handling infra run command");
                handle_infra_run(args)?;
                return Ok(true);
            }
            InfraSubCommands::ListHosts => {
                info!("'list-hosts' flag was provided");
                debug!("Displaying list of hosts");
                println!("infinity,\ntrinityhelios");
                return Ok(true);
            }
        }
    } else {
        return Ok(false);
    }

}
