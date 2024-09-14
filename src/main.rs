mod commands;
mod controller;
mod handlers;
mod logger;

use color_eyre::eyre::Result;
use controller::init_cli_controller;

fn main() -> Result<()> {
    init_cli_controller()?;

    Ok(())
}
