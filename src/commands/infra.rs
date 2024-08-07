use clap::{Args, Subcommand, ValueEnum};

#[derive(Args)]
pub struct InfraCommand {

    #[command(subcommand)]
    pub command: Option<InfraSubCommands>,

    /// See list of supported hosts for infra setup
    #[arg(short = 'l', long, exclusive = true)]
    pub list_hosts: bool,
}

#[derive(Subcommand)]
pub enum InfraSubCommands {
    /// Run infra setups
    Run(InfraRunArgs),
}

#[derive(Args)]
pub struct InfraRunArgs {

    /// Specify the host
    #[arg(long, hide_possible_values = true)]
    pub host: Hosts,

    /// Specify the setup flag
    #[arg(long,
        required_if_eq("host", "trinity-helios"),
        hide_possible_values = true
    )]
    pub extra_flag: Option<SetupExtraFlags>,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Hosts {
    TrinityHelios,
    Infinity,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SetupExtraFlags {
    PartOne,
    PartTwo,
}
