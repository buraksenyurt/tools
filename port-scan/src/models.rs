use colorized::{Color, Colors};
use std::time::Duration;

/// Models for port scanning results

/// Status of a scanned port
/// Open: The port is open and accepting connections.
/// Closed: The port is closed and not accepting connections.
/// Filtered: The port is filtered by a firewall or other network device.
#[derive(Debug, PartialEq, Eq)]
pub enum PortStatus {
    Open,
    Closed,
    Filtered,
}

impl std::fmt::Display for PortStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            PortStatus::Open => "Open",
            PortStatus::Closed => "Closed",
            PortStatus::Filtered => "Filtered",
        };
        write!(f, "{}", status_str)
    }
}

/// Information about a scanned port
/// port: The port number.
/// status: The status of the port (open, closed, filtered).
/// service: The service running on the port (if known).
/// response_time: The time taken to receive a response from the port (if applicable).
#[derive(Debug, PartialEq, Eq)]
pub struct PortInfo {
    pub port: u16,
    pub status: PortStatus,
    pub service: Option<String>,
    pub response_time: Option<Duration>,
}

impl std::fmt::Display for PortInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!(
            "Port: {:<10}\tStatus: {:15}\tService: {:<25}\tResponse Time: {:<10}",
            self.port,
            self.status,
            self.service.as_deref().unwrap_or("Unknown"),
            self.response_time
                .map(|d| format!("{:?}", d))
                .unwrap_or("N/A".into())
        );
        match self.status {
            PortStatus::Open => write!(f, "{}", text.color(Colors::BrightGreenFg)),
            PortStatus::Closed => write!(f, "{}", text.color(Colors::BrightRedFg)),
            PortStatus::Filtered => write!(f, "{}", text.color(Colors::BrightYellowFg)),
        }
    }
}
