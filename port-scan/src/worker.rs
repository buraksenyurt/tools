use std::time::Duration;

use colorized::{Color, Colors};

use crate::scanner::Scanner;

/// Start scanning the given IP address for the specified ports with a timeout.
/// # Arguments
/// * `ip` - The target IP address as a string.
/// * `ports` - A vector of port numbers to scan.
/// * `timeout` - The timeout duration for each port scan.
/// # Returns
/// * `Vec<PortInfo>` - A vector of `PortInfo` structs containing the results of the scan.
pub fn start_scan(ip: &str, ports: Vec<u16>, timeout: Duration) {
    println!(
        "{}",
        format!("Scanning {} for ports: {:?}", ip, ports).color(Colors::BrightCyanFg)
    );
    let scanner = Scanner::new(ip.parse().unwrap(), timeout);
    for port in ports {
        let result = scanner.scan_port(port);
        println!("{}", result);
    }
}

/// Start scanning a range of ports on the given IP address with a timeout.
/// # Arguments
/// * `ip` - The target IP address as a string.
/// * `start_port` - The starting port number of the range to scan.
/// * `end_port` - The ending port number of the range to scan.
pub fn start_scan_range(ip: &str, start_port: u16, end_port: u16, timeout: Duration) {
    println!(
        "{}",
        format!("Scanning {} from {} to {}", ip, start_port, end_port).color(Colors::BrightCyanFg)
    );
    let scanner = Scanner::new(ip.parse().unwrap(), timeout);
    for port in start_port..=end_port {
        let result = scanner.scan_port(port);
        println!("{}", result);
    }
}
