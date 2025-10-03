use colorized::{Color, Colors};
use std::env;
use std::thread;
use std::time::Duration;
use sysinfo::{Disks, System};

fn main() {
    clear_screen();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--help" {
        show_help();
        return;
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    print_common_info();
    println!(
        "{} {}",
        format!("{:<20}", "Number of CPUs:").color(Colors::BrightCyanFg),
        sys.cpus().len()
    );
    print_memory_info(&mut sys);
    print_disks(&mut sys);
    print_process_info(&mut sys);
}

fn print_disks(sys: &mut System) {
    sys.refresh_all();
    println!("{}", "\nDisks:".color(Colors::BrightYellowFg));
    for disk in Disks::new_with_refreshed_list().iter() {
        println!(
            "{} {} MB total, {} MB free, {} MB used",
            format!("{:<20}", disk.name().to_string_lossy()).color(Colors::BrightWhiteFg),
            format!("{:>10}", disk.total_space() / 1024 / 1024).color(Colors::BrightGreenFg),
            format!("{:>10}", disk.available_space() / 1024 / 1024).color(Colors::BrightRedFg),
            format!(
                "{:>10}",
                (disk.total_space() - disk.available_space()) / 1024 / 1024
            )
            .color(Colors::BrightYellowFg)
        );
    }
}

fn print_common_info() {
    println!("{}\n", "System Information:".color(Colors::BrightMagentaFg));
    println!(
        "{} {:?}",
        format!("{:<20}", "Name:").color(Colors::BrightCyanFg),
        System::name().unwrap_or("Unknown".into())
    );
    println!(
        "{} {:?}",
        format!("{:<20}", "Kernel Version:").color(Colors::BrightCyanFg),
        System::kernel_version().unwrap_or("Unknown".into())
    );
    println!(
        "{} {:?}",
        format!("{:<20}", "OS Version:").color(Colors::BrightCyanFg),
        System::os_version().unwrap_or("Unknown".into())
    );
    println!(
        "{} {:?}",
        format!("{:<20}", "Host Name:").color(Colors::BrightCyanFg),
        System::host_name().unwrap_or("Unknown".into())
    );
}

fn print_memory_info(sys: &mut System) {
    thread::sleep(Duration::from_millis(1000));
    sys.refresh_all();
    println!(
        "{} {} Mb",
        format!("{:<20}", "Total memory:").color(Colors::BrightCyanFg),
        format!("{:>10}", sys.total_memory() / 1024 / 1024).color(Colors::BrightGreenFg)
    );
    println!(
        "{} {} Mb",
        format!("{:<20}", "Used memory:").color(Colors::BrightCyanFg),
        format!("{:>10}", sys.used_memory() / 1024 / 1024).color(Colors::BrightRedFg)
    );
    println!(
        "{} {} Mb",
        format!("{:<20}", "Free memory:").color(Colors::BrightCyanFg),
        format!("{:>10}", sys.free_memory() / 1024 / 1024).color(Colors::BrightGreenFg)
    );
    println!(
        "{} {} Mb",
        format!("{:<20}", "Total swap:").color(Colors::BrightCyanFg),
        format!("{:>10}", sys.total_swap() / 1024 / 1024).color(Colors::BrightGreenFg)
    );
    println!(
        "{} {} Mb",
        format!("{:<20}", "Used swap:").color(Colors::BrightCyanFg),
        format!("{:>10}", sys.used_swap() / 1024 / 1024).color(Colors::BrightRedFg)
    );
}

fn print_process_info(sys: &mut System) {
    println!(
        "{}",
        "\nTop 5 Processes (Sorted by CPU Usage):\n".color(Colors::BrightMagentaFg)
    );
    thread::sleep(Duration::from_millis(1000));
    sys.refresh_all();
    let mut processes = sys.processes().iter().collect::<Vec<_>>();

    processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());

    for (pid, process) in processes.iter().take(5) {
        println!(
            "[{}]\t {:<30} CPU: {}% Mem: {} KB",
            pid.to_string().color(Colors::BrightYellowFg),
            process
                .name()
                .to_string_lossy()
                .to_string()
                .color(Colors::BrightWhiteFg),
            format!("{:>6.2}", process.cpu_usage()).color(Colors::BrightRedFg),
            format!("{:>10}", process.memory() / 1024).color(Colors::BrightGreenFg),
        );
    }
}

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

fn show_help() {
    println!(
        "{}",
        "SysTower - System Information Tool".color(Colors::BrightGreenFg)
    );
    println!("{}", "-".repeat(34).color(Colors::BrightGreenFg));
    println!();
    println!("{}", "What am I doing?".color(Colors::BrightYellowFg));
    println!();
    println!("This program displays comprehensive system information including:");
    println!(
        "  {} System details (OS, kernel, hostname)",
        "-".color(Colors::BrightGreenFg)
    );
    println!("  {} CPU information", "•".color(Colors::BrightGreenFg));
    println!(
        "  {} Memory usage (RAM and swap)",
        "-".color(Colors::BrightGreenFg)
    );
    println!(
        "  {} Disk space information",
        "-".color(Colors::BrightGreenFg)
    );
    println!(
        "  {} Top 5 processes by CPU usage",
        "-".color(Colors::BrightGreenFg)
    );
    println!();
    println!("{}", "Usage:".color(Colors::BrightCyanFg));
    println!("  sys-tower           - Display system information");
    println!("  sys-tower --help    - Show this help message");
    println!();
    println!("The output is colorized for better readability and includes real-time");
    println!("system metrics to help you monitor your system's current state.");
}
