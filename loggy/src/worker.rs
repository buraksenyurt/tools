//! Worker module containing functions for reading, filtering, and counting log entries.

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::NaiveDateTime;
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use regex::Regex;

use crate::counter::Counts;

/// Reads a log file and returns its lines as a vector of strings.
///
/// # Arguments
/// * `file_path` - A string slice that holds the path to the log file.
/// # Returns
/// * `Result<Vec<String>, std::io::Error>` - A result containing a vector of log lines or an I/O error.
/// # Errors
/// * Returns an error if the file cannot be opened or read.
pub fn read_log_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map_while(Result::ok).collect();
    Ok(lines)
}

/// Filters log lines that match a given regex pattern.
///
/// # Arguments
/// * `logs` - A slice of strings representing log lines.
/// * `pattern` - A string slice that holds the regex pattern to match.
/// # Returns
/// * `Vec<String>` - A vector of log lines that match the pattern.
pub fn filter_logs_by_pattern(logs: &[String], pattern: &str) -> Vec<String> {
    let insensitive_pattern = format!("(?i){}", pattern);
    let regex = Regex::new(&insensitive_pattern).unwrap();
    logs.iter()
        .filter(|line| regex.is_match(line))
        .cloned()
        .collect()
}

/// Filters log lines that match a given regex pattern using parallel processing.
///
/// # Arguments
/// * `logs` - A slice of strings representing log lines.
/// * `pattern` - A string slice that holds the regex pattern to match.
/// * `chunk_size` - The size of chunks to process in parallel.
/// # Returns
/// * `Vec<String>` - A vector of log lines that match the pattern.
pub fn filter_logs_by_pattern_parallel(
    logs: &[String],
    pattern: &str,
    chunk_size: usize,
) -> Vec<String> {
    let insensitive_pattern = format!("(?i){}", pattern);
    let regex = Regex::new(&insensitive_pattern).unwrap();

    logs.par_chunks(chunk_size)
        .flat_map(|chunk| {
            chunk
                .iter()
                .filter(|line| regex.is_match(line))
                .cloned()
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Checks if a log line contains a timestamp in various formats.
///
/// # Arguments
/// * `line` - A string slice representing a log line.
/// # Returns
/// * `Option<String>` - An option containing the matched timestamp string if found.
fn extract_timestamp(line: &str) -> Option<String> {
    let patterns = vec![
        r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}", // 2025-10-12 10:00:01
        r"\d{4}/\d{2}/\d{2} \d{2}:\d{2}:\d{2}", // 2025/10/12 10:00:01
        r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}", // 2025-10-12T10:00:01
        r"\w{3} \d{2} \d{2}:\d{2}:\d{2}",       // Oct 12 10:00:01
    ];
    for pattern in patterns {
        let regex = Regex::new(pattern).unwrap();
        if let Some(matched) = regex.find(line) {
            return Some(matched.as_str().to_string());
        }
    }
    None
}

/// Parses a timestamp string into a `NaiveDateTime` object.
///
/// # Arguments
/// * `timestamp` - A string slice representing the timestamp.
/// # Returns
/// * `Option<NaiveDateTime>` - An option containing the parsed `NaiveDateTime` if successful.
fn parse_timestamp(timestamp: &str) -> Option<NaiveDateTime> {
    let formats = vec![
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%b %d %H:%M:%S",
    ];
    for format in formats {
        if let Ok(dt) = NaiveDateTime::parse_from_str(timestamp, format) {
            return Some(dt);
        }
    }
    None
}

/// Filters log lines that fall within a specified time range.
///
/// # Arguments
/// * `logs` - A slice of strings representing log lines.
/// * `start` - A string slice representing the start date in "YYYY-MM-DD HH:MM:SS" format.
/// * `end` - A string slice representing the end date in "YYYY-MM-DD HH:MM:SS" format.
/// # Returns
/// * `Vec<String>` - A vector of log lines that fall within the specified time range.
pub fn filter_logs_by_time_range(logs: &[String], start: &str, end: &str) -> Vec<String> {
    let start_dt = NaiveDateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S").unwrap();
    let end_dt = NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S").unwrap();

    logs.iter()
        .filter(|line| {
            if let Some(timestamp_str) = extract_timestamp(line) {
                if let Some(dt) = parse_timestamp(&timestamp_str) {
                    return dt >= start_dt && dt <= end_dt;
                }
            }
            false
        })
        .cloned()
        .collect()
}

/// Counts the occurrences of different log levels (ERROR, WARNING, INFO) in the log lines.
///
/// # Arguments
/// * `logs` - A slice of strings representing log lines.
/// # Returns
/// * `Counts` - A struct containing the total number of logs and counts for each log level.
pub fn count_logs(logs: &[String]) -> Counts {
    let insensitive_pattern = format!("(?i){}", "ERROR");
    let errors = Regex::new(&insensitive_pattern).unwrap();

    let insensitive_pattern = format!("(?i){}", "WARNING");
    let warnings = Regex::new(&insensitive_pattern).unwrap();

    let insensitive_pattern = format!("(?i){}", "INFO");
    let infos = Regex::new(&insensitive_pattern).unwrap();

    let errors_count = logs.iter().filter(|line| errors.is_match(line)).count();
    let warnings_count = logs.iter().filter(|line| warnings.is_match(line)).count();
    let infos_count = logs.iter().filter(|line| infos.is_match(line)).count();

    Counts::new(logs.len(), errors_count, warnings_count, infos_count)
}
