mod cli;
mod counter;
mod terminal;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::NaiveDateTime;
use clap::Parser;
use colorized::{Color, Colors};
use regex::Regex;

use crate::{cli::Cli, counter::Counts};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    terminal::clear_screen();

    println!(
        "{}",
        format!("{:<10} {:>20}\n", "Log file:", args.file).color(Colors::BrightCyanFg)
    );
    if let Some(pattern) = args.pattern {
        println!(
            "{}",
            format!("{:<10} {:>20}", "Pattern:", pattern).color(Colors::BrightCyanFg)
        );
        let logs = read_log_file(&args.file)?;
        let filtered_logs = filter_logs(&logs, &pattern);
        terminal::print_logs(&filtered_logs);
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

fn read_log_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map_while(Result::ok).collect();
    Ok(lines)
}

fn filter_logs(logs: &[String], pattern: &str) -> Vec<String> {
    let regex = Regex::new(pattern).unwrap();
    logs.iter()
        .filter(|line| regex.is_match(line))
        .cloned()
        .collect()
}

fn filter_logs_by_time_range(logs: &[String], start: &str, end: &str) -> Vec<String> {
    let start_dt = NaiveDateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S").unwrap();
    let end_dt = NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S").unwrap();

    logs.iter()
        .filter(|line| {
            let mut parts = line.split_whitespace();
            let date = parts.next();
            let time = parts.next();
            if let (Some(date), Some(time)) = (date, time) {
                let ts = format!("{} {}", date, time);
                if let Ok(dt) = NaiveDateTime::parse_from_str(&ts, "%Y-%m-%d %H:%M:%S") {
                    return dt >= start_dt && dt <= end_dt;
                }
            }
            false
        })
        .cloned()
        .collect()
}

fn count_logs(logs: &[String]) -> Counts {
    let errors = Regex::new(r"ERROR").unwrap();
    let warnings = Regex::new(r"WARNING").unwrap();
    let infos = Regex::new(r"INFO").unwrap();

    let errors_count = logs.iter().filter(|line| errors.is_match(line)).count();
    let warnings_count = logs.iter().filter(|line| warnings.is_match(line)).count();
    let infos_count = logs.iter().filter(|line| infos.is_match(line)).count();

    Counts::new(logs.len(), errors_count, warnings_count, infos_count)
}
