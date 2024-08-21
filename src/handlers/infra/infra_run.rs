use log::{debug, info};
use crate::{
    commands::lvl_1::infra::{Hosts, InfraRunArgs, SetupExtraFlags},
    handlers::infra::infra_run_setup_part_one::trinity_helios_part_one
};

pub fn handle_infra_run(infra_run_arg: &InfraRunArgs) {
    debug!("Handling infra run command arguments");

    match infra_run_arg.host {
        Hosts::TrinityHelios => {
            info!("Host: trinity-helios was provided");

            if let Some(extra_flag) = infra_run_arg.extra_flag {
                match extra_flag {
                    SetupExtraFlags::PartOne => {
                        info!("Extra flag: part-one was provided");
                        info!("Setting up infrastructure for Trinity Helios");
                        info!("Running Trinity Helios setup part one");
                        trinity_helios_part_one();
                    }
                    SetupExtraFlags::PartTwo => {
                        info!("Extra flag: part-two was provided");
                        info!("Setting up infrastructure for Trinity Helios");
                        info!("Running Trinity Helios setup part two");
                    }
                }
            }
        },
        Hosts::Infinity => {
            info!("Host: trinity-helios was provided");
            info!("Setting up infrastructure for Infinity");
        },
    }
}
