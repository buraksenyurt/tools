use regex::Regex;

use crate::models::{filter_info::FilterInfo, log_entry::LogEntry, log_export::LogExport};

/// Parses a log line into a LogEntry struct.
/// 
/// # Arguments
/// * `line` - A string slice that holds a single line from the log file.
/// 
/// # Returns
/// * `Option<LogEntry>` - Some(LogEntry) if parsing is successful, None otherwise.
fn parse_log_line(line: &str) -> Option<LogEntry> {
    let re = Regex::new(r"^(\S+\s+\S+)\s+(ERROR|WARNING|WARN|INFO|DEBUG)\s+(.+)$").unwrap();
    if let Some(caps) = re.captures(line) {
        return Some(LogEntry {
            timestamp: caps[1].to_string(),
            level: caps[2].to_string(),
            message: caps[3].to_string(),
        });
    }
    None
}

/// Exports the provided log lines to a JSON structure.
/// 
/// # Arguments
/// * `logs` - A slice of strings representing log lines.
/// * `filter_info` - A FilterInfo struct containing details about the applied filters.
/// 
/// # Returns
/// * `anyhow::Result<LogExport>` - The resulting JSON structure or an error.
pub fn export_logs_to_json(logs: &[String], filter_info: FilterInfo) -> anyhow::Result<LogExport> {
    let entries: Vec<LogEntry> = logs
        .iter()
        .filter_map(|line| parse_log_line(line))
        .collect();

    let export_data = LogExport {
        total_count: entries.len(),
        filter_info,
        entries,
    };
    Ok(export_data)
}
