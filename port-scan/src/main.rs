use std::time::Duration;

mod models;
mod scanner;
mod tests;
mod worker;

fn main() {
    let port_list = vec![21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 445];
    worker::start_scan("127.0.0.1", port_list, Duration::from_secs(1));
}
