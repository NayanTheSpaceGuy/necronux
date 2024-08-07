mod cli;
mod commands;
mod handlers;
mod tests;

use clap::Parser;
use cli::Cli;
use handlers::handle_cli_operations;

fn main() {
    let cli = Cli::parse();
    handle_cli_operations(&cli);
}
