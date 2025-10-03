use crate::models::*;
use colorized::{Color, Colors};

use crate::models::SystemData;

pub trait Display {
    fn display_all(&self);
}

impl Display for SystemData {
    fn display_all(&self) {
        self.system_info.display();
        self.display_cpu_info();
        self.memory_info.display();
        self.display_disk_info();
        self.display_process_info();
    }
}

impl SystemData {
    fn display_cpu_info(&self) {
        println!(
            "{} {}",
            format!("{:<20}", "Number of CPUs/Cores:").color(Colors::BrightCyanFg),
            self.cpu_count
        );

        for cpu in &self.cpu_info {
            cpu.display();
        }
    }

    fn display_disk_info(&self) {
        println!("{}", "\nDisks:".color(Colors::BrightYellowFg));
        for disk in &self.disk_info {
            disk.display();
        }
    }

    fn display_process_info(&self) {
        println!(
            "{}",
            "\nTop 5 Processes (Sorted by CPU Usage):\n".color(Colors::BrightMagentaFg)
        );
        for process in &self.process_info {
            process.display();
        }
    }
}

impl SystemInfo {
    fn display(&self) {
        println!("{}\n", "System Information:".color(Colors::BrightMagentaFg));
        println!(
            "{} {}",
            format!("{:<20}", "Name:").color(Colors::BrightCyanFg),
            self.name
        );
        println!(
            "{} {}",
            format!("{:<20}", "Kernel Version:").color(Colors::BrightCyanFg),
            self.kernel_version
        );
        println!(
            "{} {}",
            format!("{:<20}", "OS Version:").color(Colors::BrightCyanFg),
            self.os_version
        );
        println!(
            "{} {}",
            format!("{:<20}", "Host Name:").color(Colors::BrightCyanFg),
            self.host_name
        );
    }
}

impl CpuInfo {
    fn display(&self) {
        println!(
            "{} {} MHz ({} - {})",
            format!("{:<20}", self.name).color(Colors::BrightWhiteFg),
            format!("{:>10}", self.frequency).color(Colors::BrightGreenFg),
            self.vendor_id.color(Colors::BrightYellowFg),
            self.brand.color(Colors::BrightCyanFg)
        );
    }
}

impl MemoryInfo {
    fn display(&self) {
        println!(
            "{} {} MB",
            format!("{:<20}", "Total memory:").color(Colors::BrightCyanFg),
            format!("{:>10}", self.total_memory / 1024 / 1024).color(Colors::BrightGreenFg)
        );
        println!(
            "{} {} MB",
            format!("{:<20}", "Used memory:").color(Colors::BrightCyanFg),
            format!("{:>10}", self.used_memory / 1024 / 1024).color(Colors::BrightRedFg)
        );
        println!(
            "{} {} MB",
            format!("{:<20}", "Free memory:").color(Colors::BrightCyanFg),
            format!("{:>10}", self.free_memory / 1024 / 1024).color(Colors::BrightGreenFg)
        );
        println!(
            "{} {} MB",
            format!("{:<20}", "Total swap:").color(Colors::BrightCyanFg),
            format!("{:>10}", self.total_swap / 1024 / 1024).color(Colors::BrightGreenFg)
        );
        println!(
            "{} {} MB",
            format!("{:<20}", "Used swap:").color(Colors::BrightCyanFg),
            format!("{:>10}", self.used_swap / 1024 / 1024).color(Colors::BrightRedFg)
        );
    }
}

impl DiskInfo {
    fn display(&self) {
        println!(
            "{} {} MB total, {} MB free, {} MB used",
            format!("{:<20}", self.name).color(Colors::BrightWhiteFg),
            format!("{:>10}", self.total_space / 1024 / 1024).color(Colors::BrightGreenFg),
            format!("{:>10}", self.available_space / 1024 / 1024).color(Colors::BrightRedFg),
            format!("{:>10}", self.used_space / 1024 / 1024).color(Colors::BrightYellowFg)
        );
    }
}

impl ProcessInfo {
    fn display(&self) {
        println!(
            "[{}]\t {:<50} CPU: {}% Mem: {} KB",
            self.pid.color(Colors::BrightYellowFg),
            self.name.color(Colors::BrightWhiteFg),
            format!("{:>6.2}", self.cpu_usage).color(Colors::BrightRedFg),
            format!("{:>10}", self.memory / 1024).color(Colors::BrightGreenFg),
        );
    }
}
