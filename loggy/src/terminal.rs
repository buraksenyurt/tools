/// Terminal-related utilities for loggy.


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