mod controller;
mod commands;
mod logger;
mod handlers;
mod tests;

use controller::init_cli_controller;

fn main() {
    init_cli_controller();
}
