use std::{fs::File, io::{BufRead, BufReader}};

use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loggy - A simple log parser and analyzer tool running in the terminal.");

    let file_path = "api-runtime.log";
    let logs = read_log_file(file_path)?;

    let error_logs = filter_logs(&logs, r"ERROR");
    for log in error_logs {
        println!("{}", log);
    }

    Ok(())
}

fn read_log_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().filter_map(Result::ok).collect();
    Ok(lines)
}

fn filter_logs(logs:&[String],pattern:&str)->Vec<String>{
    let regex=Regex::new(pattern).unwrap();
    logs.iter().filter(|line|regex.is_match(line)).cloned().collect()
}
