use utility::{Colorize, Colors};

/// Struct to hold counts of different log levels.
#[derive(Debug)]
pub struct Counts {
    /// Total number of log entries.
    pub total: usize,
    /// Number of error log entries.
    pub error: usize,
    /// Number of warning log entries.
    pub warning: usize,
    /// Number of info log entries.
    pub info: usize,
    /// Timestamp of the log entry.
    pub timestamp: f32,
}

impl Counts {
    /// Creates a new `Counts` instance.
    ///
    /// # Arguments
    /// * `total` - Total number of log entries.
    /// * `error` - Number of error log entries.
    /// * `warning` - Number of warning log entries.
    /// * `info` - Number of info log entries.
    /// # Returns
    /// * `Counts` - A new instance of the `Counts` struct.
    pub fn new(total: usize, error: usize, warning: usize, info: usize) -> Self {
        Counts {
            total,
            error,
            warning,
            info,
            timestamp: chrono::Local::now().timestamp_millis() as f32 / 1000.0,
        }
    }

    /// Prints the counts in a formatted manner with colors.
    pub fn print(&self) {
        println!(
            "{}",
            format!("{:<10} {:>10}", "Total:", self.total).colorize(Colors::LightBlue)
        );
        println!(
            "{}",
            format!("{:<10} {:>10}", "Error:", self.error).colorize(Colors::LightRed)
        );
        println!(
            "{}",
            format!("{:<10} {:>10}", "Warning:", self.warning).colorize(Colors::LightYellow)
        );
        println!(
            "{}",
            format!("{:<10} {:>10}", "Info:", self.info).colorize(Colors::LightBlue)
        );
    }
}
