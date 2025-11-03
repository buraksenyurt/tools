use crate::models::*;
use utility_lib::{Colorize, Colors};

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
            format!("{:<20}", "Number of CPUs/Cores:").colorize(Colors::LightCyan),
            self.cpu_count
        );

        for cpu in &self.cpu_info {
            cpu.display();
        }
    }

    fn display_disk_info(&self) {
        println!("{}", "\nDisks:".colorize(Colors::LightYellow));
        for disk in &self.disk_info {
            disk.display();
        }
    }

    fn display_process_info(&self) {
        println!(
            "{}",
            "\nTop 5 Processes (Sorted by CPU Usage):\n".colorize(Colors::LightMagenta)
        );
        for process in &self.process_info {
            process.display();
        }
    }
}

impl SystemInfo {
    fn display(&self) {
        println!("{}\n", "System Information:".colorize(Colors::LightMagenta));
        println!(
            "{} {}",
            format!("{:<20}", "Name:").colorize(Colors::LightCyan),
            self.name
        );
        println!(
            "{} {}",
            format!("{:<20}", "Kernel Version:").colorize(Colors::LightCyan),
            self.kernel_version
        );
        println!(
            "{} {}",
            format!("{:<20}", "OS Version:").colorize(Colors::LightCyan),
            self.os_version
        );
        println!(
            "{} {}",
            format!("{:<20}", "Host Name:").colorize(Colors::LightCyan),
            self.host_name
        );
    }
}

impl CpuInfo {
    fn display(&self) {
        println!(
            "{} {} MHz ({} - {})",
            format!("{:<20}", self.name).colorize(Colors::LightWhite),
            format!("{:>10}", self.frequency).colorize(Colors::LightGreen),
            self.vendor_id.colorize(Colors::LightYellow),
            self.brand.colorize(Colors::LightCyan)
        );
    }
}

impl MemoryInfo {
    fn display(&self) {
        println!(
            "{} {} MB",
            format!("{:<20}", "Total memory:").colorize(Colors::LightCyan),
            format!("{:>10}", self.total_memory / 1024 / 1024).colorize(Colors::LightGreen)
        );
        println!(
            "{} {} MB",
            format!("{:<20}", "Used memory:").colorize(Colors::LightCyan),
            format!("{:>10}", self.used_memory / 1024 / 1024).colorize(Colors::LightRed)
        );
        println!(
            "{} {} MB",
            format!("{:<20}", "Free memory:").colorize(Colors::LightCyan),
            format!("{:>10}", self.free_memory / 1024 / 1024).colorize(Colors::LightGreen)
        );
        println!(
            "{} {} MB",
            format!("{:<20}", "Total swap:").colorize(Colors::LightCyan),
            format!("{:>10}", self.total_swap / 1024 / 1024).colorize(Colors::LightGreen)
        );
        println!(
            "{} {} MB",
            format!("{:<20}", "Used swap:").colorize(Colors::LightCyan),
            format!("{:>10}", self.used_swap / 1024 / 1024).colorize(Colors::LightRed)
        );
    }
}

impl DiskInfo {
    fn display(&self) {
        println!(
            "{} {} MB total, {} MB free, {} MB used",
            format!("{:<20}", self.name).colorize(Colors::LightWhite),
            format!("{:>10}", self.total_space / 1024 / 1024).colorize(Colors::LightGreen),
            format!("{:>10}", self.available_space / 1024 / 1024).colorize(Colors::LightRed),
            format!("{:>10}", self.used_space / 1024 / 1024).colorize(Colors::LightYellow)
        );
    }
}

impl ProcessInfo {
    fn display(&self) {
        println!(
            "[{}]\t {:<50} CPU: {}% Mem: {} KB",
            self.pid.colorize(Colors::LightYellow),
            self.name.colorize(Colors::LightWhite),
            format!("{:>6.2}", self.cpu_usage).colorize(Colors::LightRed),
            format!("{:>10}", self.memory / 1024).colorize(Colors::LightGreen),
        );
    }
}
