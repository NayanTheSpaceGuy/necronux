pub mod global_args;
pub mod infra;

use crate::cli::{Cli, Commands};

pub fn handle_cli_operations(cli: &Cli) {

    // Handle global arguments
    global_args::handle_global_args(&cli.global);

    // Handle the specific commands
    match &cli.command {
        Commands::Infra(infra_command) => infra::handle_infra(infra_command),
        Commands::System(_) => {
            // system::handle_system(system_command)
            println!("System command not yet implemented");
        },
        Commands::App(_) => {
            // app::handle_app(app_command)
            println!("App command not yet implemented");
        },
    }
}
