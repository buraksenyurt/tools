use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "fico - A simple file copy tool",
    version,
    about = "A simple file copy tool with progress bar support."
)]
pub struct Args {
    #[arg(short, long, help = "Source file path")]
    pub source: PathBuf,
    #[arg(short, long, help = "Destination file path")]
    pub destination: PathBuf,
    #[arg(short, long, help = "Force overwrite existing files")]
    pub force: bool,
    #[arg(short, long, help = "Verify file integrity after copy")]
    pub verify: bool,
}
