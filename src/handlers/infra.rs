use crate::commands::lvl_1::infra::{InfraCommand, InfraSubCommands, InfraRunArgs, Hosts, SetupExtraFlags};

pub fn handle_infra(command: &InfraCommand) {
    handle_infra_args(command);
    handle_infra_command(command);
}

pub fn handle_infra_args(infra_command: &InfraCommand) {
    match infra_command.infra_list_hosts {
        true => println!("infinity,\ntrinityhelios"),
        false => {}
    }
}

pub fn handle_infra_command(infra_command: &InfraCommand) {
    if let Some(subcommand) = &infra_command.infra_subcommand {
        match subcommand {
            InfraSubCommands::Run(args) => handle_infra_run(args),
        }
    }
}

fn handle_infra_run(infra_run_arg: &InfraRunArgs) {
    match infra_run_arg.host {
        Hosts::TrinityHelios => {
            if let Some(extra_flag) = infra_run_arg.extra_flag {
                match extra_flag {
                    SetupExtraFlags::PartOne => println!(
                    "Setting up infrastructure for Trinity Helios\nRunning Trinity Helios setup part one"),
                    SetupExtraFlags::PartTwo => println!(
                    "Setting up infrastructure for Trinity Helios\nRunning Trinity Helios setup part two"),
                }
            }
        },
        Hosts::Infinity => {
            println!("Setting up infrastructure for Infinity");
        },
    }
}
