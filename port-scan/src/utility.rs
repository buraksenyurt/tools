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