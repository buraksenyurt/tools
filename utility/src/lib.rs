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

/// Trait to add colorization functionality to strings.
pub trait Colorize {
    /// Colors the string with the specified color code.
    /// # Arguments
    /// * `color_code` - The color code to apply.
    /// # Returns
    /// * `String` - The colorized string.
    fn colorize(&self, color_code: Colors) -> String;
}

/// Implementation of the Colorize trait for all types that implement Display.
impl<T> Colorize for T
where
    T: std::fmt::Display,
{
    /// Colors the string with the specified color code.
    /// # Arguments
    /// * `color_code` - The color code to apply.
    /// # Returns
    /// * `String` - The colorized string.
    fn colorize(&self, color_code: Colors) -> String {
        format!("\x1B[{}m{}\x1B[0m", color_code as u8, self)
    }
}

/// Enum representing various terminal colors.
pub enum Colors {
    /// Black color
    Black = 30,
    /// Light Black color
    LightBlack = 90,
    /// Red color
    Red = 31,
    /// Light Red color
    LightRed = 91,
    /// Green color
    Green = 32,
    /// Light Green color
    LightGreen = 92,
    /// Yellow color
    Yellow = 33,
    /// Light Yellow color
    LightYellow = 93,
    /// Blue color  
    Blue = 34,
    /// Light Blue color
    LightBlue = 94,
    /// Magenta color
    Magenta = 35,
    /// Light Magenta color
    LightMagenta = 95,
    /// Cyan color
    Cyan = 36,
    /// Light Cyan color
    LightCyan = 96,
    /// White color
    White = 37,
    /// Light White color
    LightWhite = 97,
}
