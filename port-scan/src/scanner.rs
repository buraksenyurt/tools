/// A simple port scanner implementation in Rust.
/// This module defines a `Scanner` struct that can scan a given IP address for open ports within a specified timeout duration.
use std::{
    net::{IpAddr, SocketAddr, TcpStream},
    time::{Duration, Instant},
};

use crate::models::{PortInfo, PortStatus};

/// Scanner
/// A struct representing a port scanner.
/// It contains the target IP address and the timeout duration for each port scan.
pub struct Scanner {
    pub target: IpAddr,
    pub timeout: Duration,
}

/// Implementation of the Scanner struct.
impl Scanner {
    /// Create a new Scanner instance.
    /// # Arguments
    /// * `target` - The target IP address to scan.
    /// * `timeout` - The timeout duration for each port scan.
    pub fn new(target: IpAddr, timeout: Duration) -> Self {
        Scanner { target, timeout }
    }

    /// Scan a specific port on the target IP address.
    ///
    /// This function attempts to connect to the specified port and returns information about the port's status.
    /// 
    /// # Arguments
    /// * `port` - The port number to scan.
    /// # Returns
    /// * `PortInfo` - A struct containing information about the scanned port.
    pub fn scan_port(&self, port: u16) -> PortInfo {
        let socket = SocketAddr::new(self.target, port);
        let start = Instant::now();

        match TcpStream::connect_timeout(&socket, self.timeout) {
            Ok(_) => PortInfo {
                port,
                status: PortStatus::Open,
                service: get_service_name(port),
                response_time: Some(start.elapsed()),
            },
            Err(_) => PortInfo {
                port,
                status: PortStatus::Closed,
                service: None,
                response_time: None,
            },
        }
    }
}

/// Get common service names for well-known ports.
/// This function returns the service name associated with a given port number, if known.
/// For example, port 80 is commonly associated with HTTP.
/// Returns `None` if the port is not recognized.
/// # Arguments
/// * `port` - The port number to look up.
/// # Returns
/// * `Option<String>` - The service name if known, otherwise `None`.
fn get_service_name(port: u16) -> Option<String> {
    match port {
        21 => Some("FTP".into()),
        22 => Some("SSH".into()),
        23 => Some("Telnet".into()),
        25 => Some("SMTP".into()),
        53 => Some("DNS".into()),
        80 => Some("HTTP".into()),
        110 => Some("POP3".into()),
        135 => Some("RPC Endpoint Mapper".into()),
        139 => Some("NetBIOS".into()),
        143 => Some("IMAP".into()),
        443 => Some("HTTPS".into()),
        445 => Some("SMB".into()),
        993 => Some("IMAPS".into()),
        995 => Some("POP3S".into()),
        3306 => Some("MySQL".into()),
        3389 => Some("RDP".into()),
        5357 => Some("Web Services on Devices(WSD)".into()),
        5900 => Some("VNC".into()),
        5432 => Some("PostgreSQL".into()),
        6379 => Some("Redis".into()),
        8080 => Some("HTTP Proxy".into()),
        8443 => Some("HTTPS-Alt".into()),
        15672 => Some("RabbitMQ".into()),
        _ => None,
    }
}
