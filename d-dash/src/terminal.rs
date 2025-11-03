use crate::report::Report;
use utility::{Colorize, Colors};

pub fn print_header() {
    println!("{}", "*".repeat(80).colorize(Colors::LightYellow));
    println!("{}", "D-Dash".colorize(Colors::LightWhite));
    println!(
        "{}",
        "A simple directory and file analysis tool written in Rust.".colorize(Colors::LightWhite)
    );
    println!("{}", "*".repeat(80).colorize(Colors::LightYellow));
    println!();
}

pub fn print_dashboard(report: &Report) {
    println!("{}", "*".repeat(80).colorize(Colors::LightYellow));
    println!("{}", "Summary".colorize(Colors::LightWhite));
    println!();
    println!(
        "{}",
        format!("Scanned Path: {}", report.scanned_path).colorize(Colors::LightCyan)
    );
    println!(
        "{}",
        format!("Deep Level: {}", report.level).colorize(Colors::LightCyan)
    );
    println!(
        "{}",
        format!("Total Files: {}", report.total_files).colorize(Colors::LightCyan)
    );
    println!(
        "{}",
        format!("Total Directories: {}", report.total_directories).colorize(Colors::LightCyan)
    );
    println!(
        "{}",
        format!("Total Size: {} bytes", report.total_size).colorize(Colors::LightCyan)
    );
    if let Some(ref file) = report.most_huge_file {
        println!(
            "{}",
            format!(
                "Most Huge File: {} ({} bytes)",
                file, report.most_huge_file_size
            )
            .colorize(Colors::LightCyan)
        );
    } else {
        println!("{}", "Most Huge File: None".colorize(Colors::LightCyan));
    }
    if let Some(ref ext) = report.most_used_extension {
        println!(
            "{}",
            format!(
                "Most Used Extension: {} ({} files)",
                ext, report.most_used_extension_count
            )
            .colorize(Colors::LightCyan)
        );
    } else {
        println!(
            "{}",
            "Most Used Extension: None".colorize(Colors::LightCyan)
        );
    }
    println!("{}", "*".repeat(80).colorize(Colors::LightYellow));
    println!();
}

pub fn print_entities(entities: &[crate::entity::Entity]) {
    println!(
        "{}",
        format!(
            "{:<25}\t{:<5}\t{:<20}\t{:>10}",
            "Name", "Type", "Extension", "Size"
        )
        .colorize(Colors::LightMagenta)
    );
    println!("{}", "-".repeat(80).colorize(Colors::LightYellow));
    for entity in entities {
        println!("{}", entity);
    }
}
