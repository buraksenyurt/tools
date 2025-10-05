use std::{fs, io};
mod entity;
mod report;
mod terminal;
mod worker;

use colorized::{Color, Colors};

use crate::report::Report;
fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    println!(
        "{}",
        format!("Investigating path: {}", path)
            .as_str()
            .color(Colors::BrightGreenFg)
    );

    let metadata = fs::metadata(&path)?;

    if !metadata.is_dir() {
        eprintln!(
            "{}",
            format!("The provided path is not a directory: {}", path)
                .as_str()
                .color(Colors::BrightRedFg)
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
                    .color(Colors::BrightRedFg)
            );
            return Ok(());
        }
    };

    let report = Report::new(path, 0, &entities);
    terminal::print_dashboard(&report);
    terminal::print_entities(&entities);

    Ok(())
}
