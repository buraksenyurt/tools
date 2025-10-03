use std::time::Duration;

use crate::scanner::Scanner;

/// Start scanning the given IP address for the specified ports with a timeout.
/// # Arguments
/// * `ip` - The target IP address as a string.
/// * `ports` - A vector of port numbers to scan.
/// * `timeout` - The timeout duration for each port scan.
pub fn start_scan(ip: &str, ports: Vec<u16>, timeout: Duration) {
    let scanner = Scanner::new(ip.parse().unwrap(), timeout);
    for port in ports {
        let result = scanner.scan_port(port);
        println!("{}", result);
    }
}
