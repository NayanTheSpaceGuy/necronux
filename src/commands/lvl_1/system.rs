use clap::Args;

#[derive(Args, Debug)]
pub struct SystemCommand {

    /// See list of available system utilities
    #[arg(short = 'l', long, exclusive = true)]
    pub list_utils: bool,
}
