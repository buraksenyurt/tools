use std::{thread, time::Duration};

use utility::{Colorize, Colors};

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
        format!("Scanning {} for ports: {:?}", ip, ports).colorize(Colors::LightCyan)
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
        format!("Scanning {} from {} to {}", ip, start_port, end_port).colorize(Colors::LightCyan)
    );
    let scanner = Scanner::new(ip.parse().unwrap(), timeout);
    for port in start_port..=end_port {
        let result = scanner.scan_port(port);
        println!("{}", result);
    }
}

/// Start scanning a range of ports on the given IP address in parallel using multiple threads.
///
/// # Arguments
///
/// * `ip` - The target IP address as a string.
/// * `start_port` - The starting port number of the range to scan.
/// * `end_port` - The ending port number of the range to scan.
pub fn start_scan_range_parallel(ip: &str, start_port: u16, end_port: u16, timeout: Duration) {
    println!(
        "{}",
        format!("Scanning {} from {} to {}", ip, start_port, end_port).colorize(Colors::LightCyan)
    );
    let ip_address = ip.parse().unwrap();
    let mut handles = vec![];
    for thread_id in 0..4 {
        let thread_start = start_port + thread_id * ((end_port - start_port) / 4);
        let thread_end = if thread_id == 3 {
            end_port
        } else {
            thread_start + ((end_port - start_port) / 4) - 1
        };

        let handle = thread::spawn(move || {
            let scanner = Scanner::new(ip_address, timeout);

            for port in thread_start..=thread_end {
                let result = scanner.scan_port(port);
                println!("{}", result);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

/// Validate command-line arguments for the port scanner.
/// # Arguments
/// * `args` - A slice of command-line arguments.
/// # Returns
/// * `Result<(&str, &str), String>` - Ok with IP address and ports/range if valid, Err with error message if invalid.
pub fn validate_arguments(args: &[String]) -> Result<(&str, &str, bool), String> {
    if args.len() < 3 {
        return Err("Invalid arguments".into());
    }
    let ip_string = &args[1];
    let ports_or_range = &args[2];
    let use_multithreading = args.len() > 3 && args[3] == "-p";

    match ip_string.parse::<std::net::IpAddr>() {
        Ok(_) => {}
        Err(_) => return Err(format!("Error: Invalid IP address provided: {}", ip_string)),
    }

    if ports_or_range.contains('-') {
        // Validate port range
        let port_parts: Vec<&str> = ports_or_range.split('-').collect();
        if port_parts.len() != 2 {
            return Err("Error: Invalid port range format. Use 'start-end'".into());
        }

        let start_port = port_parts[0]
            .parse::<u16>()
            .map_err(|_| format!("Error: Invalid start port number: {}", port_parts[0]))?;
        let end_port = port_parts[1]
            .parse::<u16>()
            .map_err(|_| format!("Error: Invalid end port number: {}", port_parts[1]))?;

        if start_port > end_port {
            return Err("Error: Start port must be less than or equal to end port".into());
        }
    } else if ports_or_range.contains(',') {
        let ports: Vec<&str> = ports_or_range.split(',').collect();
        for port_str in ports {
            port_str
                .trim()
                .parse::<u16>()
                .map_err(|_| format!("Error: Invalid port number: {}", port_str.trim()))?;
        }
    } else {
        ports_or_range
            .parse::<u16>()
            .map_err(|_| format!("Error: Invalid port number: {}", ports_or_range))?;
    }

    Ok((ip_string, ports_or_range, use_multithreading))
}
