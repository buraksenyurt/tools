use serde::Serialize;

/// Information about the filters applied to log entries.
/// This struct captures details about the filtering criteria used
/// when processing log files, such as patterns and time ranges.
#[derive(Debug, Serialize)]
pub struct FilterInfo {
    /// The pattern used to filter log entries.
    pub pattern: Option<String>,
    /// The start time for the log entries to include.
    pub start_time: Option<String>,
    /// The end time for the log entries to include.
    pub end_time: Option<String>,
}
