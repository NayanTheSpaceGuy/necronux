use clap::Subcommand;
use crate::commands::lvl_1::{
    app::AppCommand,
    infra::InfraCommand,
    system::SystemCommand
};

#[derive(Subcommand, Debug)]
pub enum NecronuxCommand {

    #[command(arg_required_else_help = true)]
    /// Setup infrastructure, resources and their configurations
    Infra(InfraCommand),

    #[command(arg_required_else_help = true)]
    /// Setup system tweaks and execute system utilities
    System(SystemCommand),

    #[command(arg_required_else_help = true)]
    /// Setup application or batch of applications with their configurations
    App(AppCommand),
}
