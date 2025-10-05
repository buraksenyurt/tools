use crate::entity::{Entity, Type};

#[derive(Debug)]
pub struct Report {
    pub total_files: usize,
    pub total_directories: usize,
    pub total_size: u64,
    pub most_huge_file: Option<String>,
    pub most_huge_file_size: u64,
    pub most_used_extension: Option<String>,
    pub most_used_extension_count: usize,
    pub scanned_path: String,
    pub level: usize,
}

impl Report {
    pub fn new(scanned_path: String,entities: &[Entity]) -> Self {
        let mut total_files = 0;
        let mut total_directories = 0;
        let mut total_size = 0;
        let mut most_huge_file = None;
        let mut most_huge_file_size = 0;
        let mut extension_count = std::collections::HashMap::new();

        for entity in entities {
            match entity.entity_type {
                Type::File => {
                    total_files += 1;
                    total_size += entity.size;
                    if entity.size > most_huge_file_size {
                        most_huge_file_size = entity.size;
                        most_huge_file = Some(entity.name.clone());
                    }
                    if let Some(ext) = entity.extension.clone() {
                        *extension_count.entry(ext).or_insert(0) += 1;
                    }
                }
                Type::Directory => total_directories += 1,
            }
        }

        let (most_used_extension, most_used_extension_count) = extension_count
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .unwrap_or(("None".to_string(), 0));

        Self {
            total_files,
            total_directories,
            total_size,
            most_huge_file,
            most_huge_file_size,
            most_used_extension: Some(most_used_extension),
            most_used_extension_count,
            scanned_path,
            level: total_directories + 1,
        }
    }
}
