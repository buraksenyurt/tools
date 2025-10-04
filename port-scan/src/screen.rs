use colorized::{Color, Colors};

pub fn show_info(messages: Option<&str>) {
    clear_screen();
    show_banner();
    show_usage();
    if let Some(msg) = messages {
        println!("{}", msg.color(Colors::BrightRedFg));
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status();
    } else {
        let _ = std::process::Command::new("clear").status();
    }
    print!("\x1B[2J\x1B[1;1H");
}

fn show_banner() {
    println!("{}", "=".repeat(80).color(Colors::BrightYellowFg));
    println!(
        "{}",
        "      Port Scanner Tool       ".color(Colors::BrightYellowFg)
    );
    println!(
        "{}",
        "Its a simple port scanner written in Rust.".color(Colors::BrightYellowFg)
    );
    println!("{}", "=".repeat(80).color(Colors::BrightYellowFg));
}

fn show_usage() {
    println!(
        "{}",
        "Usage: port-scan <IP_ADDRESS> [PORTS] [RANGE_START RANGE_END]".color(Colors::BrightCyanFg)
    );
    println!(
        "{}",
        "Example: port-scan 127.0.0.1 80,100,135".color(Colors::BrightCyanFg)
    );
    println!(
        "{}",
        "Example: port-scan 127.0.0.1 130-140".color(Colors::BrightCyanFg)
    );
    println!(
        "{}",
        "Example: port-scan 127.0.0.1 135".color(Colors::BrightCyanFg)
    );
    println!("{}", "=".repeat(80).color(Colors::BrightCyanFg));
    println!();
}
