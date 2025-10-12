mod cli;
mod counter;
mod terminal;
mod worker;

use clap::Parser;
use colorized::{Color, Colors};

use crate::{cli::Cli, worker::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    terminal::clear_screen();

    println!(
        "{}",
        format!("{:<10} {:>20}\n", "Log file:", args.file).color(Colors::BrightCyanFg)
    );
    if let Some(pattern) = args.pattern {
        if args.parallel {
            println!(
                "{}",
                format!(
                    "Filtering logs by pattern '{}' in parallel (chunk size: {})",
                    pattern, args.chunk_size
                )
                .color(Colors::BrightCyanFg)
            );
            let logs = read_log_file(&args.file)?;
            let filtered_logs = filter_logs_by_pattern_parallel(&logs, &pattern, args.chunk_size);
            terminal::print_logs(&filtered_logs);
            return Ok(());
        } else {
            println!(
                "{}",
                format!("Filtering logs by pattern '{}'", pattern).color(Colors::BrightCyanFg)
            );
            let logs = read_log_file(&args.file)?;
            let filtered_logs = filter_logs_by_pattern(&logs, &pattern);
            terminal::print_logs(&filtered_logs);
            return Ok(());
        }
    }

    if let (Some(start), Some(end)) = (args.start, args.end) {
        println!(
            "{}",
            format!("Filtering logs from {} to {}", start, end).color(Colors::BrightCyanFg)
        );
        let logs = read_log_file(&args.file)?;
        let time_filtered_logs = filter_logs_by_time_range(&logs, &start, &end);
        terminal::print_logs(&time_filtered_logs);
    }

    if args.counts {
        let logs = read_log_file(&args.file)?;
        let counts = count_logs(&logs);
        counts.print();
    }

    Ok(())
}
