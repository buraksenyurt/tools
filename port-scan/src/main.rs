use std::{env, time::Duration};

mod models;
mod scanner;
mod screen;
mod tests;
mod worker;

fn main() {
    screen::show_info();

    let args = env::args();
    if args.len() < 2 {
        screen::show_info();
        return;
    }

    let port_list = vec![21, 22, 80, 110, 135, 139, 143, 445];
    worker::start_scan("127.0.0.1", port_list, Duration::from_millis(500));
    worker::start_scan_range("127.0.0.1", 130, 140, Duration::from_millis(500));
}
