#[cfg(test)]
use crate::Cli;

#[test]
pub fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
