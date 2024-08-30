use color_eyre::eyre::Result;
use log::{debug, info};
use crate::{
    commands::lvl_1::infra::{InfraCommand, InfraSubCommands},
    handlers::infra::infra_run::handle_infra_run
};

pub fn handle_infra(command: &InfraCommand) -> Result<()> {

    debug!("Handling infra command arguments");
    handle_infra_args(command)?;
    debug!("Handling infra command subcommands");
    handle_infra_command(command)?;

    Ok(())
}

pub fn handle_infra_args(infra_command: &InfraCommand) -> Result<()> {

    match infra_command.infra_list_hosts {
        true => println!("infinity,\ntrinityhelios"),
        false => {}
    }

    Ok(())
}

pub fn handle_infra_command(infra_command: &InfraCommand) -> Result<()> {

    if let Some(subcommand) = &infra_command.infra_subcommand {
        match subcommand {
            InfraSubCommands::Run(args) => {
                info!("'infra run' command was provided");
                debug!("Handling infra run command");
                handle_infra_run(args)?;
            }
        }
    }

    Ok(())
}
