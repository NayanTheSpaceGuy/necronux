use clap::Args;

#[derive(Args)]
pub struct AppCommand {

    /// See list of available system utilities
    #[arg(short = 'l', long, exclusive = true)]
    pub list_apps: bool,
}
