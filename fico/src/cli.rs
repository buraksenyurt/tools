use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "fico - A simple file copy tool",
    version,
    about = "A simple file copy tool with progress bar support."
)]
struct Args {
    #[arg(short, long, help = "Source file path")]
    source: PathBuf,
    #[arg(short, long, help = "Destination file path")]
    destination: PathBuf,
    #[arg(short, long, help = "Force overwrite existing files")]
    force: bool,
}
