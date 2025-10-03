use std::time::Duration;

/// Models for port scanning results


/// Status of a scanned port
/// Open: The port is open and accepting connections.
/// Closed: The port is closed and not accepting connections.
/// Filtered: The port is filtered by a firewall or other network device.
#[derive(Debug, PartialEq, Eq)]
pub enum PortStatus{
    Open,
    Closed,
    Filtered
}

/// Information about a scanned port
/// port: The port number.
/// status: The status of the port (open, closed, filtered).
/// service: The service running on the port (if known).
/// response_time: The time taken to receive a response from the port (if applicable).
#[derive(Debug, PartialEq, Eq)]
pub struct PortInfo{
    pub port: u16,
    pub status: PortStatus,
    pub service: Option<String>,
    pub response_time : Option<Duration>
}