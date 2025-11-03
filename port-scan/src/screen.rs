use utility::clear_screen;
use utility::{Colorize, Colors};

/// Display information and usage instructions for the port scanner tool.
///
/// # Arguments
/// * `messages` - An optional string slice containing messages to display (e.g., errors).
pub fn show_info(messages: Option<&str>) {
    clear_screen();
    show_banner();
    show_usage();
    if let Some(msg) = messages {
        println!("{}", msg.colorize(Colors::LightRed));
    }
}

/// Display the banner for the port scanner tool.
fn show_banner() {
    println!("{}", "=".repeat(80).colorize(Colors::LightYellow));
    println!(
        "{}",
        "      Port Scanner Tool       ".colorize(Colors::LightYellow)
    );
    println!(
        "{}",
        "Its a simple port scanner written in Rust.".colorize(Colors::LightYellow)
    );
    println!("{}", "=".repeat(80).colorize(Colors::LightYellow));
}

/// Display usage instructions for the port scanner tool.
fn show_usage() {
    println!(
        "{}",
        "Usage: port-scan <IP_ADDRESS> [PORTS] [RANGE_START RANGE_END] [OPTIONS]"
            .colorize(Colors::LightCyan)
    );
    println!(
        "{}",
        "Example: port-scan 127.0.0.1 80,100,135".colorize(Colors::LightCyan)
    );
    println!(
        "{}",
        "Example: port-scan 127.0.0.1 130-140".colorize(Colors::LightCyan)
    );
    println!(
        "{}",
        "Example: port-scan 127.0.0.1 130-140 -p #Multithreading".colorize(Colors::LightCyan)
    );
    println!(
        "{}",
        "Example: port-scan 127.0.0.1 135".colorize(Colors::LightCyan)
    );
    println!("{}", "=".repeat(80).colorize(Colors::LightCyan));
    println!();
}
