use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::NaiveDateTime;
use regex::Regex;

use crate::counter::Counts;

pub fn read_log_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map_while(Result::ok).collect();
    Ok(lines)
}

pub fn filter_logs_by_pattern(logs: &[String], pattern: &str) -> Vec<String> {
    let regex = Regex::new(pattern).unwrap();
    logs.iter()
        .filter(|line| regex.is_match(line))
        .cloned()
        .collect()
}

pub fn filter_logs_by_time_range(logs: &[String], start: &str, end: &str) -> Vec<String> {
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

pub fn count_logs(logs: &[String]) -> Counts {
    let errors = Regex::new(r"ERROR").unwrap();
    let warnings = Regex::new(r"WARNING").unwrap();
    let infos = Regex::new(r"INFO").unwrap();

    let errors_count = logs.iter().filter(|line| errors.is_match(line)).count();
    let warnings_count = logs.iter().filter(|line| warnings.is_match(line)).count();
    let infos_count = logs.iter().filter(|line| infos.is_match(line)).count();

    Counts::new(logs.len(), errors_count, warnings_count, infos_count)
}
