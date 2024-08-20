#[cfg(test)]
use clap::CommandFactory;
#[cfg(test)]
use crate::controller::Cli;

#[test]
pub fn verify_cli() {
    Cli::command().debug_assert();
}
