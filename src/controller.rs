use clap::Parser;
use log::info;
use crate::{
    commands::lvl_0::{global_args::GlobalArgs, necronux::NecronuxCommand},
    logger::init_logger,
    handlers::init::init_handlers
};

pub fn init_cli_controller () {

    let cli = Cli::parse();

    init_logger(&cli.global_args);
    info!("Initialized logger");

    info!("Initializing handlers");
    init_handlers(&cli);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub necronux_command: Option<NecronuxCommand>,

    #[command(flatten)]
    pub global_args: GlobalArgs,
}
