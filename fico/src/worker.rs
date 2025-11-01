use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

use colorized::Color;
use indicatif::ProgressBar;
use sha2::{Digest, Sha256};
use utility::clear_screen;

const BUFFER_SIZE: usize = 8192;

pub fn copy_file(source: &PathBuf, destination: &PathBuf, force: bool) -> anyhow::Result<()> {
    if !source.exists() {
        return Err(anyhow::anyhow!(
            "Source file `{}` does not exist",
            source.display()
        ));
    }

    if destination.exists() && !force {
        return Err(anyhow::anyhow!(
            "Destination file `{}` already exists. Use --force to overwrite.",
            destination.display()
        ));
    }

    clear_screen();

    let source_file = File::open(source)?;
    let file_size = source_file.metadata()?.len();
    let info = format!(
        "Copying from `{}` to `{}`",
        source.display(),
        destination.display()
    );
    println!("{}", info.color(colorized::Colors::BrightYellowFg));
    let info = format!("File size: {} bytes", file_size);
    println!("{}", info.color(colorized::Colors::BrightYellowFg));

    let destination_file = File::create(destination)?;

    let mut reader = BufReader::new(source_file);
    let mut writer = BufWriter::new(destination_file);

    let progress_bar = ProgressBar::new(file_size);
    progress_bar.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        writer.write_all(&buffer[..bytes_read])?;
        progress_bar.inc(bytes_read as u64);
    }

    progress_bar.finish_with_message("Copy complete");
    writer.flush()?;

    Ok(())
}

pub fn verify_integrity(source: &PathBuf, destination: &PathBuf) -> anyhow::Result<bool> {
    let source_checksum = calculate_checksum(source)?;
    let destination_checksum = calculate_checksum(destination)?;

    Ok(source_checksum == destination_checksum)
}

fn calculate_checksum(path: &PathBuf) -> anyhow::Result<u32> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut hasher = Sha256::new();

    println!(
        "{}",
        format!("Calculating checksum for `{}`", path.display())
            .color(colorized::Colors::BrightYellowFg)
    );
    let file_size = file.metadata()?.len();
    let progress_bar = ProgressBar::new(file_size);
    progress_bar.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
        progress_bar.inc(bytes_read as u64);
    }

    let checksum = hasher.finalize();
    Ok(checksum
        .iter()
        .fold(0u32, |acc, &byte| acc.wrapping_add(byte as u32)))
}

#[cfg(test)]
mod tests {
    use super::copy_file;
    use std::path::PathBuf;

    #[test]
    fn test_source_not_found() {
        let source = PathBuf::from("non_existent_file.txt");
        let destination = PathBuf::from("destination.txt");
        let result = copy_file(&source, &destination, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }
}
