use colorized::{Color, Colors};

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status();
    } else {
        let _ = std::process::Command::new("clear").status();
    }
    print!("\x1B[2J\x1B[1;1H");
}

pub fn show_loading_message() {
    println!();
    println!(
        "{}",
        "Calculating system information...".color(Colors::BrightYellowFg)
    );
    println!(
        "{}",
        "Please wait while we gather data...".color(Colors::BrightCyanFg)
    );
    println!();
}

pub fn show_help() {
    println!(
        "{}",
        "SysTower - System Information Tool".color(Colors::BrightGreenFg)
    );
    println!("{}", "-".repeat(34).color(Colors::BrightGreenFg));
    println!();
    println!("{}", "What am I doing?".color(Colors::BrightYellowFg));
    println!();
    println!("This program displays comprehensive system information including:");
    println!(
        "  {} System details (OS, kernel, hostname)",
        "-".color(Colors::BrightGreenFg)
    );
    println!("  {} CPU information", "-".color(Colors::BrightGreenFg));
    println!(
        "  {} Memory usage (RAM and swap)",
        "-".color(Colors::BrightGreenFg)
    );
    println!(
        "  {} Disk space information",
        "-".color(Colors::BrightGreenFg)
    );
    println!(
        "  {} Top 5 processes by CPU usage",
        "-".color(Colors::BrightGreenFg)
    );
    println!();
    println!("{}", "Usage:".color(Colors::BrightCyanFg));
    println!("  sys-tower           - Display system information");
    println!("  sys-tower --help    - Show this help message");
    println!();
    println!("The output is colorized for better readability and includes real-time");
    println!("system metrics to help you monitor your system's current state.");
}
