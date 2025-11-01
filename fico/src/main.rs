mod cli;
mod worker;

use clap::Parser;
use cli::*;
use colorized::{Color, Colors};
use worker::*;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match copy_file(&args.source, &args.destination, args.force) {
        Ok(_) => println!(
            "{}",
            "File copied successfully.".color(Colors::BrightGreenFg)
        ),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
