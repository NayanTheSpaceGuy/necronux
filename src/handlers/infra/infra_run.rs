use color_eyre::eyre::Result;
use log::{info, debug};
use crate::{
    commands::lvl_1::infra::{Hosts, InfraRunArgs, SetupExtraFlags},
    handlers::infra::run_setup_trinity_helios::{ trinity_helios_part_one, trinity_helios_part_two},
};

pub fn handle_infra_run(infra_run_arg: &InfraRunArgs) -> Result<()> {

    match infra_run_arg.host {
        Hosts::TrinityHelios => {
            info!("Host: trinity-helios was provided");

            if let Some(extra_flag) = infra_run_arg.extra_flag {
                match extra_flag {
                    SetupExtraFlags::PartOne => {
                        info!("Extra flag: part-one was provided");
                        info!("Setting up infrastructure for Trinity Helios");
                        debug!("Running Trinity Helios setup part one script");
                        trinity_helios_part_one()?;
                    }
                    SetupExtraFlags::PartTwo => {
                        info!("Extra flag: part-two was provided");
                        info!("Setting up infrastructure for Trinity Helios");
                        debug!("Running Trinity Helios setup part two script");
                        trinity_helios_part_two()?;
                    }
                }
            }
        },
        Hosts::Infinity => {
            info!("Host: infinity was provided");
            info!("Setting up infrastructure for Infinity");
            info!("Infinity setup not implemented yet");
        },
    }

    Ok(())
}
