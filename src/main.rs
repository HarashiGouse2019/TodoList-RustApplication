//Starting up the main application
use log::info;

mod todo_cli;

fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting up Todo-List Cli App...");
    todo_cli::run().expect("Failed to run application!");
    Ok(())
}