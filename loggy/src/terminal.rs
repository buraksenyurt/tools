//! Terminal-related utilities for loggy.

use std::collections::VecDeque;

use colorized::{Color, Colors};
use textplots::{Chart, Plot, Shape};

use crate::counter::Counts;

/// Clears the terminal screen.
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

/// Prints the provided log lines to the terminal.
///
/// # Arguments
/// * `logs` - A slice of strings representing log lines.
pub fn print_logs(logs: &[String]) {
    for line in logs {
        println!("{}", line);
    }
}

/// Draws live statistics of log levels over time using text-based plots.
///
/// # Arguments
/// * `stats` - A deque of `Counts` representing log level statistics over time.
pub fn draw_live_stats(stats: &VecDeque<Counts>) {
    clear_screen();
    if stats.is_empty() {
        println!("No log entries yet.");
        return;
    }

    let error_diff: Vec<(f32, f32)> = stats
        .iter()
        .map(|s| (s.timestamp, s.error as f32))
        .collect();

    println!(
        "{}",
        "ERROR Trends Over Time"
            .to_string()
            .color(Colors::BrightRedFg)
    );
    Chart::new(120, 20, 0.0, stats.len() as f32)
        .lineplot(&Shape::Lines(&error_diff))
        .display();

    let warning_diff: Vec<(f32, f32)> = stats
        .iter()
        .map(|s| (s.timestamp, s.warning as f32))
        .collect();

    println!(
        "{}",
        "WARNING Trends Over Time"
            .to_string()
            .color(Colors::BrightYellowFg)
    );
    Chart::new(120, 20, 0.0, stats.len() as f32)
        .lineplot(&Shape::Lines(&warning_diff))
        .display();

    if let Some(counts) = stats.back() {
        counts.print();
    }
}
