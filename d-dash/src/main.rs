use std::{fs, io};
mod entity;
mod report;
mod terminal;
mod worker;

use utility::{Colorize, Colors};
use utility::clear_screen;

use crate::report::Report;
fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    println!(
        "{}",
        format!("Investigating path: {}", path)
            .as_str()
            .colorize(Colors::LightGreen)
    );

    let metadata = fs::metadata(&path)?;

    if !metadata.is_dir() {
        eprintln!(
            "{}",
            format!("The provided path is not a directory: {}", path)
                .as_str()
                .colorize(Colors::LightRed)
        );
        return Ok(());
    }

    let process_result = worker::process_directory(&path, &mut std::collections::HashSet::new());
    let entities = match process_result {
        Ok(ents) => ents,
        Err(e) => {
            eprintln!(
                "{}",
                format!("Error processing directory: {}", e)
                    .as_str()
                    .colorize(Colors::LightRed)
            );
            return Ok(());
        }
    };

    let report = Report::new(path, &entities);
    clear_screen();
    terminal::print_header();
    terminal::print_entities(&entities);
    terminal::print_dashboard(&report);

    Ok(())
}
