// src/main.rs

// Declare the commands module to include all the files inside the 'commands' directory.
mod commands;

use std::env;
use anyhow::Result;

fn main() -> Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check the second argument to decide which command to run
    match args.get(1).map(|s| s.as_str()) {
        Some("format") => {
            commands::format::execute()?;
        }
        Some("lint") => {
            commands::lint::execute()?;
        }
        _ => {
            println!("Unknown command or no command provided.");
            println!("Usage: cargo run -- <command>");
            println!("Available commands: format, lint");
        }
    }

    Ok(())
}
