use std::{fs, io};

use colorized::{Color, Colors};
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

    let entries = fs::read_dir(&path)?;
    for entry in entries {
        match entry {
            Ok(entry) => {
                let file_type = entry.file_type()?;
                let file_name = entry.file_name().into_string().unwrap_or_default();
                if file_type.is_dir() {
                    println!(
                        "{}",
                        format!("Dir:  {}", file_name)
                            .as_str()
                            .color(Colors::BrightYellowFg)
                    );
                } else if file_type.is_file() {
                    let metadata = fs::metadata(entry.path())?;
                    println!(
                        "{}",
                        format!("File: {}, {} bytes", file_name, metadata.len())
                            .as_str()
                            .color(Colors::BrightGreenFg)
                    );
                } else {
                    println!(
                        "{}",
                        format!("Other: {}", file_name)
                            .as_str()
                            .color(Colors::BrightYellowFg)
                    );
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

    Ok(())
}
