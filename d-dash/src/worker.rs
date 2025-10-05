use std::{collections::HashSet, fs, path::PathBuf};

use crate::entity::{Entity, Type};

pub fn process_directory(
    path: &str,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<Entity>, std::io::Error> {
    let mut entities = Vec::new();
    let canonical = fs::canonicalize(path)?;
    if !visited.insert(canonical.clone()) {
        return Ok(entities);
    }

    let entries = fs::read_dir(&canonical)?;
    for entry in entries {
        if let Ok(entry) = entry {
            let full_path = entry.path();
            let meta = fs::symlink_metadata(&full_path)?;
            let file_type = meta.file_type();
            let file_name = entry.file_name().into_string().unwrap_or_default();

            if file_type.is_dir() {
                entities.push(Entity::new(file_name.clone(), Type::Directory, 0, None));
                let sub = process_directory(full_path.to_str().unwrap(), visited)?;
                entities.extend(sub);
            } else if file_type.is_file() {
                let size = meta.len();
                let extension = full_path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|s| s.to_string());
                entities.push(Entity::new(file_name, Type::File, size, extension));
            }
        }
    }
    Ok(entities)
}
