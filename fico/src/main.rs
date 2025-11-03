mod cli;
mod worker;

use clap::Parser;
use cli::*;
use utility::{Colorize, Colors};
use worker::*;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match copy_file(&args.source, &args.destination, args.force) {
        Ok(_) => println!(
            "{}",
            "File copied successfully.".colorize(Colors::LightGreen)
        ),
        Err(e) => eprintln!("Error: {}", e),
    }

    if args.verify {
        match verify_integrity(&args.source, &args.destination) {
            Ok(true) => println!(
                "{}",
                "File integrity verified successfully.".colorize(Colors::LightGreen)
            ),
            Ok(false) => eprintln!(
                "{}",
                "File integrity verification failed.".colorize(Colors::LightRed)
            ),
            Err(e) => eprintln!("Error during integrity verification: {}", e),
        }
    }

    Ok(())
}
