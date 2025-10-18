use serde::{Deserialize, Serialize};

/// Represents a single log entry in the log file.
#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// The timestamp of the log entry.
    pub timestamp: String,
    /// The log level of the entry (e.g., INFO, ERROR, WARN).
    pub level: String,
    /// The log message.
    pub message: String,
}
