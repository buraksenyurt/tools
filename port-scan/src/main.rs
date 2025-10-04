use std::{env, net::IpAddr, time::Duration};

mod models;
mod scanner;
mod screen;
mod tests;
mod worker;

fn main() {
    let args = env::args();
    if args.len() != 3 {
        screen::show_info(None);
        return;
    }
    let mut ip_string = String::new();
    let mut ports_or_range = String::new();
    for arg in args.enumerate() {
        match arg.0 {
            1 => ip_string = arg.1,
            2 => ports_or_range = arg.1,
            _ => {}
        }
    }

    match ip_string.parse::<IpAddr>() {
        Ok(_) => {}
        Err(_) => {
            let error_message = format!("Error: Invalid IP address provided: {}", ip_string);
            screen::show_info(Some(&error_message));
            std::process::exit(1);
        }
    };

    if ports_or_range.contains('-') {
        let port_range: Vec<u16> = ports_or_range
            .split('-')
            .filter_map(|p| p.parse::<u16>().ok())
            .collect();
        if !port_range.is_empty() {
            worker::start_scan_range(
                &ip_string,
                port_range[0],
                port_range[1],
                Duration::from_millis(500),
            );
        }
        return;
    }

    if ports_or_range.contains(',') {
        let port_list: Vec<u16> = ports_or_range
            .split(',')
            .filter_map(|p| p.parse::<u16>().ok())
            .collect();
        if !port_list.is_empty() {
            worker::start_scan(&ip_string, port_list, Duration::from_millis(500));
        }
    }

    let port: u16 = ports_or_range.parse().unwrap_or_else(|_| {
        let error_message = format!("Error: Invalid port provided: {}", ports_or_range);
        screen::show_info(Some(&error_message));
        std::process::exit(1);
    });
    worker::start_scan(&ip_string, vec![port], Duration::from_millis(500));
}
