mod cli;
mod counter;
mod export;
mod models;
mod terminal;
mod worker;

use clap::Parser;
use utility::{Colorize, Colors, clear_screen};

use crate::{cli::Cli, models::filter_info::FilterInfo, worker::*};

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    clear_screen();

    println!(
        "{}",
        format!("{:<10} {:>20}\n", "Log file:", args.file).colorize(Colors::LightCyan)
    );

    if args.counts {
        let logs = read_log_file(&args.file)?;
        let counts = count_logs(&logs);
        counts.print();
        return Ok(());
    }

    if args.watch {
        println!(
            "{}",
            format!("Watching log file: {}", args.file).colorize(Colors::LightCyan)
        );
        watch_real_time(&args.file)?;
        return Ok(());
    }

    let mut filtered_logs: Vec<String> = Vec::new();
    let mut args_pattern: Option<String> = None;
    let mut start_time: Option<String> = None;
    let mut end_time: Option<String> = None;

    if let Some(pattern) = args.pattern {
        args_pattern = Some(pattern.clone());
        if args.parallel {
            println!(
                "{}",
                format!(
                    "Filtering logs by pattern '{}' in parallel (chunk size: {})",
                    pattern, args.chunk_size
                )
                .colorize(Colors::LightCyan)
            );
            let logs = read_log_file(&args.file)?;
            filtered_logs = filter_logs_by_pattern_parallel(&logs, &pattern, args.chunk_size);
            terminal::print_logs(&filtered_logs);
        } else {
            println!(
                "{}",
                format!("Filtering logs by pattern '{}'", pattern).colorize(Colors::LightCyan)
            );
            let logs = read_log_file(&args.file)?;
            filtered_logs = filter_logs_by_pattern(&logs, &pattern);
            terminal::print_logs(&filtered_logs);
        }
    }

    if let (Some(start), Some(end)) = (args.start, args.end) {
        start_time = Some(start.clone());
        end_time = Some(end.clone());
        println!(
            "{}",
            format!("Filtering logs from {} to {}", start, end).colorize(Colors::LightCyan)
        );
        let logs = read_log_file(&args.file)?;
        filtered_logs = filter_logs_by_time_range(&logs, &start, &end);
        terminal::print_logs(&filtered_logs);
    }

    if let Some(export_path) = args.export {
        println!(
            "{}",
            format!("Exporting filtered logs to {}", export_path).colorize(Colors::LightCyan)
        );
        let filter_info = FilterInfo {
            pattern: args_pattern,
            start_time,
            end_time,
        };
        export_logs_to_json_file(&filtered_logs, filter_info, &export_path)?;
        println!(
            "{}",
            format!("Logs exported successfully to {}", export_path).colorize(Colors::LightGreen)
        );
    }

    Ok(())
}
