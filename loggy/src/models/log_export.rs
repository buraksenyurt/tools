use serde::Serialize;

use super::{filter_info::FilterInfo, log_entry::LogEntry};

/// Represents the exported log data, including total count,
/// filtering information, and the log entries themselves.
#[derive(Debug, Serialize)]
pub struct LogExport {
    /// The total number of log entries exported.
    pub total_count: usize,
    /// Information about the filters applied to the log entries.
    pub filter_info: FilterInfo,
    /// The list of log entries.
    pub entries: Vec<LogEntry>,
}
