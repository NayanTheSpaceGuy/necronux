use crate::commands::infra::{InfraCommand, InfraSubCommands, InfraRunArgs, Hosts, SetupExtraFlags};

pub fn handle_infra_command(command: &InfraCommand) {
    if command.list_hosts {
        list_supported_hosts();
    } else if let Some(subcommand) = &command.command {
        match subcommand {
            InfraSubCommands::Run(args) => handle_infra_run(args),
        }
    } else {
        println!("No infra subcommand provided. Use --help for usage information.");
    }
}

fn list_supported_hosts() {
    println!("Supported hosts for infra setup:");
    println!("- Trinity Helios");
    println!("- Infinity");
}

fn handle_infra_run(args: &InfraRunArgs) {
    match args.host {
        Hosts::TrinityHelios => {
            println!("Setting up infrastructure for Trinity Helios");
            if let Some(extra_flag) = args.extra_flag {
                match extra_flag {
                    SetupExtraFlags::PartOne => println!("Running Trinity Helios setup part one"),
                    SetupExtraFlags::PartTwo => println!("Running Trinity Helios setup part two"),
                }
            } else {
                println!("Error: extra_flag is required for Trinity Helios setup");
            }
        },
        Hosts::Infinity => {
            println!("Setting up infrastructure for Infinity");
            // Infinity setup doesn't require an extra flag
        },
    }
}

pub fn handle_infra(command: &InfraCommand) {
    handle_infra_command(command);
}
