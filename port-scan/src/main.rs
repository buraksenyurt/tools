use std::{env, time::Duration};

mod models;
mod scanner;
mod screen;
mod tests;
mod worker;

const TIMEOUT_MS: u64 = 1000;

fn main() {
    screen::show_info(None);
    let args = env::args();
    let (ip_string, ports_or_range, use_multithreading) =
        match worker::validate_arguments(args.collect::<Vec<String>>().as_slice()) {
            Ok((ip, ports_or_range, use_multithreading)) => (
                ip.to_string(),
                ports_or_range.to_string(),
                use_multithreading,
            ),
            Err(e) => {
                screen::show_info(Some(&e));
                std::process::exit(1);
            }
        };

    if ports_or_range.contains('-') {
        let port_range: Vec<u16> = ports_or_range
            .split('-')
            .filter_map(|p| p.parse::<u16>().ok())
            .collect();
        if use_multithreading {
            worker::start_scan_range_parallel(
                &ip_string,
                port_range[0],
                port_range[1],
                Duration::from_millis(TIMEOUT_MS),
            );
            return;
        } else {
            worker::start_scan_range(
                &ip_string,
                port_range[0],
                port_range[1],
                Duration::from_millis(TIMEOUT_MS),
            );
        }
        return;
    }

    if ports_or_range.contains(',') {
        let port_list: Vec<u16> = ports_or_range
            .split(',')
            .filter_map(|p| p.parse::<u16>().ok())
            .collect();
        worker::start_scan(&ip_string, port_list, Duration::from_millis(TIMEOUT_MS));
        return;
    }

    let port: u16 = ports_or_range.parse().unwrap();
    worker::start_scan(&ip_string, vec![port], Duration::from_millis(TIMEOUT_MS));
}
