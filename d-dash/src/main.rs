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

    Ok(())
}
