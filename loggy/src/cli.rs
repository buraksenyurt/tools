use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[arg(short, long, help = "Path to the log file")]
    pub file: String,

    #[arg(
        short,
        long,
        help = "Pattern to search for in the log file",
        default_value = "ERROR"
    )]
    pub pattern: Option<String>,

    #[arg(short,long,help = "Start date filter in YYYY-MM-DD HH:MM:SS format")]
    pub start: Option<String>,

    #[arg(short,long,help = "End date filter in YYYY-MM-DD HH:MM:SS format")]
    pub end: Option<String>,
}
