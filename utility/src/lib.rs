/// Clears the terminal screen
///
/// This function attempts to clear the terminal screen by executing
/// the appropriate command based on the operating system.
///
/// On Windows, it runs the `cls` command, while on Unix-like systems,
/// it runs the `clear` command. After executing the command, it also prints ANSI
/// escape codes to ensure the screen is cleared in terminals that
/// support them.
/// 
/// # Examples
/// ```
/// use utility::clear_screen;
/// clear_screen();
/// ```
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
