mod cli;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::NaiveDateTime;
use clap::Parser;
use regex::Regex;

use crate::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    println!("Log file: {}", args.file);
    if let Some(pattern) = args.pattern {
        println!("Pattern: {}", pattern);
        let logs = read_log_file(&args.file)?;
        let filtered_logs = filter_logs(&logs, &pattern);
        print_logs(&filtered_logs);
    }

    if let (Some(start), Some(end)) = (args.start, args.end) {
        println!("Filtering logs from {} to {}", start, end);
        let logs = read_log_file(&args.file)?;
        let time_filtered_logs = filter_logs_by_time_range(&logs, &start, &end);
        print_logs(&time_filtered_logs);
    }

    Ok(())
}

fn read_log_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().filter_map(Result::ok).collect();
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
            if let Some(ts) = line.split_whitespace().next() {
                if let Ok(dt) = NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S") {
                    return dt >= start_dt && dt <= end_dt;
                }
            }
            false
        })
        .cloned()
        .collect()
}

fn print_logs(logs: &[String]) {
    for line in logs {
        println!("{}", line);
    }
}
