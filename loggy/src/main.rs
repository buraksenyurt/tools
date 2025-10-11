mod cli;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

fn print_logs(logs: &[String]) {
    for line in logs {
        println!("{}", line);
    }
}
