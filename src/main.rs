mod cli;
mod core;

use log::info;

fn main() {
    env_logger::init();

    const VERSION: &str = "2.0.0";
    info!("Starting email-reminder application - v{}", VERSION);

    // Create global context
    let mut context = match core::Context::new() {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Failed to initialize: {}", e);
            std::process::exit(1);
        }
    };

    // Run CLI
    if let Err(e) = cli::main(&mut context) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
