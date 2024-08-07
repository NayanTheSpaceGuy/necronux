use clap::{Args, Parser, Subcommand};
use crate::commands::{app, infra, system};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub global: GlobalArgs,
}

#[derive(Subcommand)]
pub enum Commands {

    #[command(arg_required_else_help = true)]
    /// Setup infrastructure, resources and their configurations
    Infra(infra::InfraCommand),

    #[command(arg_required_else_help = true)]
    /// Setup system tweaks and execute system utilities
    System(system::SystemCommand),

    #[command(arg_required_else_help = true)]
    /// Setup application or batch of applications with their configurations
    App(app::AppCommand),
}

#[derive(Args, Debug)]
pub struct GlobalArgs {

    #[arg(short, long, global = true)]
    /// Print debugging information
    pub debug: bool,

    #[arg(short, long, global = true)]
    /// Print even more information
    pub verbose: bool,
}
