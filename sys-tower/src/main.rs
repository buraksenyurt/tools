use std::thread;
use std::time::Duration;
use sysinfo::{Disks, System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    print_common_info();
    println!("{:<20} {}", "Number of CPUs:", sys.cpus().len());
    print_memory_info(&mut sys);
    print_disks(&mut sys);
    print_process_info(&mut sys);
}

fn print_disks(sys: &mut System) {
    sys.refresh_all();
    println!("Disks:");
    for disk in Disks::new_with_refreshed_list().iter() {
        println!(
            "{:<20} {:>10} MB",
            disk.name().to_string_lossy(),
            disk.total_space() / 1024 / 1024
        );
    }
}

fn print_common_info() {
    println!("System information:\n");
    println!(
        "{:<20} {:?}",
        "Name:",
        System::name().unwrap_or("Unknown".into())
    );
    println!(
        "{:<20} {:?}",
        "Kernel Version:",
        System::kernel_version().unwrap_or("Unknown".into())
    );
    println!(
        "{:<20} {:?}",
        "OS Version:",
        System::os_version().unwrap_or("Unknown".into())
    );
    println!(
        "{:<20} {:?}",
        "Host Name:",
        System::host_name().unwrap_or("Unknown".into())
    );
}

fn print_memory_info(sys: &mut System) {
    thread::sleep(Duration::from_millis(1000));
    sys.refresh_all();
    println!("{:<20} {:>10} Mb", "Total memory:", sys.total_memory() / 1024 / 1024);
    println!("{:<20} {:>10} Mb", "Used memory:", sys.used_memory() / 1024 / 1024);
    println!("{:<20} {:>10} Mb", "Free memory:", sys.free_memory() / 1024 / 1024);
    println!("{:<20} {:>10} Mb", "Total swap:", sys.total_swap() / 1024 / 1024);
    println!("{:<20} {:>10} Mb", "Used swap:", sys.used_swap() / 1024 / 1024);
}

fn print_process_info(sys: &mut System) {
    println!("\nProcess list (Sorted by CPU Usage):\n");
    thread::sleep(Duration::from_millis(1000));
    sys.refresh_all();
    let mut processes = sys.processes().iter().collect::<Vec<_>>();

    processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());

    for (pid, process) in processes.iter().take(5) {
        println!(
            "[{:>10}]\t {:<30} CPU: {:>6.2}% Mem: {:>10} KB",
            pid,
            process.name().to_string_lossy(),
            process.cpu_usage(),
            process.memory() / 1024,
        );
    }
}
