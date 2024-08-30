mod controller;
mod commands;
mod logger;
mod handlers;

use color_eyre::eyre::Result;
use controller::init_cli_controller;

fn main() -> Result<()> {

    color_eyre::install()?;

    init_cli_controller()?;

    Ok(())
}
