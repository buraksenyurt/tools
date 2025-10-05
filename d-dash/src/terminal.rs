use crate::report::Report;
use colorized::{Color, Colors};

/// Clear the terminal screen.
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

pub fn print_dashboard(report: &Report) {
    clear_screen();
    println!("{}", "*".repeat(80).color(Colors::BrightYellowFg));
    println!("{}", "D-Dash".color(Colors::BrightWhiteFg));
    println!(
        "{}",
        "A simple directory and file analysis tool written in Rust.".color(Colors::BrightWhiteFg)
    );
    println!("{}", "*".repeat(80).color(Colors::BrightYellowFg));
    println!();

    println!(
        "{}",
        format!("Scanned Path: {}", report.scanned_path).color(Colors::BrightCyanFg)
    );
    println!(
        "{}",
        format!("Deep Level: {}", report.level).color(Colors::BrightCyanFg)
    );
    println!(
        "{}",
        format!("Total Files: {}", report.total_files).color(Colors::BrightCyanFg)
    );
    println!(
        "{}",
        format!("Total Directories: {}", report.total_directories).color(Colors::BrightCyanFg)
    );
    println!(
        "{}",
        format!("Total Size: {} bytes", report.total_size).color(Colors::BrightCyanFg)
    );
    if let Some(ref file) = report.most_huge_file {
        println!(
            "{}",
            format!(
                "Most Huge File: {} ({} bytes)",
                file, report.most_huge_file_size
            )
            .color(Colors::BrightCyanFg)
        );
    } else {
        println!("{}", "Most Huge File: None".color(Colors::BrightCyanFg));
    }
    if let Some(ref ext) = report.most_used_extension {
        println!(
            "{}",
            format!(
                "Most Used Extension: {} ({} files)",
                ext, report.most_used_extension_count
            )
            .color(Colors::BrightCyanFg)
        );
    } else {
        println!(
            "{}",
            "Most Used Extension: None".color(Colors::BrightCyanFg)
        );
    }
    println!("{}", "*".repeat(80).color(Colors::BrightYellowFg));
    println!();
}

pub fn print_entities(entities: &[crate::entity::Entity]) {
    println!(
        "{}",
        format!(
            "{:<25}\t{:<5}\t{:<20}\t{:>10}",
            "Name", "Type", "Extension", "Size"
        )
        .color(Colors::BrightMagentaFg)
    );
    println!("{}", "-".repeat(80).color(Colors::BrightYellowFg));
    for entity in entities {
        println!("{}", entity);
    }
}
