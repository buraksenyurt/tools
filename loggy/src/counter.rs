use colorized::{Color, Colors};

#[derive(Debug)]
pub struct Counts {
    total: usize,
    error: usize,
    warning: usize,
    info: usize,
}

impl Counts {
    pub fn new(total: usize, error: usize, warning: usize, info: usize) -> Self {
        Counts {
            total,
            error,
            warning,
            info,
        }
    }

    pub fn print(&self) {
        println!(
            "{}",
            format!("{:<10} {:>10}", "Total:", self.total).color(Colors::BrightGreenFg)
        );
        println!(
            "{}",
            format!("{:<10} {:>10}", "Error:", self.error).color(Colors::BrightRedFg)
        );
        println!(
            "{}",
            format!("{:<10} {:>10}", "Warning:", self.warning).color(Colors::BrightYellowFg)
        );
        println!(
            "{}",
            format!("{:<10} {:>10}", "Info:", self.info).color(Colors::BrightBlueFg)
        );
    }
}
