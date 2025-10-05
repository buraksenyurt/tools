use std::fs;

use colorized::{Color, Colors};

use crate::entity::{Entity, Type};

pub fn process_directory(path: &str) -> Result<Vec<Entity>, std::io::Error> {
    let mut entities = Vec::new();
    let entries = fs::read_dir(path)?;
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
    Ok(entities)
}
