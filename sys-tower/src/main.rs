use std::thread;
use std::time::Duration;
use sysinfo::{Disks, System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    print_system_information();
    print_metrics(&sys);
    println!("Number of CPUs: {}", sys.cpus().len());
    print_disks(&mut sys);
    print_processes(&mut sys);
}

fn print_disks(sys: &mut System) {
    sys.refresh_all();
    println!("Disks");
    for disk in Disks::new_with_refreshed_list().iter() {
        println!("Refreshed Disk {disk:?} bytes");
    }
}

fn print_system_information() {
    println!("System information:");
    println!("\tName:           {:?}", System::name());
    println!("\tKernel Version: {:?}", System::kernel_version());
    println!("\tOS Version:     {:?}", System::os_version());
    println!("\tHost Name:      {:?}", System::host_name());
}

fn print_metrics(sys: &System) {
    println!("Total memory: {} Mb", sys.total_memory() / 1024 / 1024);
    println!("Used memory: {} Mb", sys.used_memory() / 1024 / 1024);
    println!("Total swap: {} Mb", sys.total_swap() / 1024 / 1024);
    println!("Used swap: {} Mb", sys.used_swap() / 1024 / 1024);
}

fn print_processes(sys: &mut System) {
    println!("Process list (Sorted by CPU Usage):");
    thread::sleep(Duration::from_millis(1000));
    sys.refresh_all();
    let mut processes = sys.processes().iter().collect::<Vec<_>>();

    processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());

    for (pid, process) in processes.iter().take(5) {
        println!(
            "[{pid}] {:?} CPU: {:6.2}% Mem: {:>8} KB, Disk:{:?}",
            process.name(),
            process.cpu_usage(),
            process.memory() / 1024,
            process.disk_usage()
        );
    }
}
