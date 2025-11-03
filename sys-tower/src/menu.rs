use utility::{Colorize, Colors};

pub fn show_loading_message() {
    println!();
    println!(
        "{}",
        "Calculating system information...".colorize(Colors::LightYellow)
    );
    println!(
        "{}",
        "Please wait while we gather data...".colorize(Colors::LightCyan)
    );
    println!();
}

pub fn show_help() {
    println!(
        "{}",
        "SysTower - System Information Tool".colorize(Colors::LightGreen)
    );
    println!("{}", "-".repeat(34).colorize(Colors::LightGreen));
    println!();
    println!("{}", "What am I doing?".colorize(Colors::LightYellow));
    println!();
    println!("This program displays comprehensive system information including:");
    println!(
        "  {} System details (OS, kernel, hostname)",
        "-".colorize(Colors::LightGreen)
    );
    println!("  {} CPU information", "-".colorize(Colors::LightGreen));
    println!(
        "  {} Memory usage (RAM and swap)",
        "-".colorize(Colors::LightGreen)
    );
    println!(
        "  {} Disk space information",
        "-".colorize(Colors::LightGreen)
    );
    println!(
        "  {} Top 5 processes by CPU usage",
        "-".colorize(Colors::LightGreen)
    );
    println!();
    println!("{}", "Usage:".colorize(Colors::LightCyan));
    println!("  sys-tower           - Display system information");
    println!("  sys-tower --help    - Show this help message");
    println!();
    println!("The output is colorized for better readability and includes real-time");
    println!("system metrics to help you monitor your system's current state.");
}
