use std::{fs, io};
mod entity;
mod report;
mod terminal;

use entity::{Entity, Type};

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

    let mut entities = Vec::new();
    let entries = fs::read_dir(&path)?;
    for entry in entries {
        match entry {
            Ok(entry) => {
                let file_type = entry.file_type()?;
                let file_name = entry.file_name().into_string().unwrap_or_default();
                if file_type.is_dir() {
                    let entity = Entity::new(file_name, Type::Directory, 0, None);
                    entities.push(entity);
                } else if file_type.is_file() {
                    let metadata = fs::metadata(entry.path())?;
                    let extension = entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|s| s.to_string());
                    let entity = Entity::new(file_name, Type::File, metadata.len(), extension);
                    entities.push(entity);
                }
            }
            Err(e) => eprintln!(
                "{}",
                format!("Error reading entry: {}", e)
                    .as_str()
                    .color(Colors::BrightRedFg)
            ),
        }
    }
    
    let report = Report::new(path, 0, &entities);
    terminal::print_dashboard(&report);
    terminal::print_entities(&entities);

    Ok(())
}
