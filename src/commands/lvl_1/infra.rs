use clap::{Args, Subcommand, ValueEnum};

#[derive(Args, Debug)]
pub struct InfraCommand {
    #[command(subcommand)]
    pub infra_subcommand: Option<InfraSubCommands>,
}

#[derive(Subcommand, Debug)]
pub enum InfraSubCommands {
    /// Run infra setups
    Run(InfraRunArgs),
    /// See list of supported hosts for infra setup
    ListHosts,
}

#[derive(Args, Debug)]
pub struct InfraRunArgs {
    /// Specify the host
    #[arg(long, hide_possible_values = true)]
    pub host: Hosts,

    /// Specify the setup flag
    #[arg(
        long,
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
