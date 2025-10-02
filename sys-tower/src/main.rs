use sysinfo::{Disks, System};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    print_system_information();
    print_metrics(&sys);

    println!("Number of CPUs: {}", sys.cpus().len());

    println!("Disks");
    for disk in Disks::new_with_refreshed_list().iter() {
        println!("Refreshed Disk {disk:?}:");
    }

    print_processes(&sys);
}

fn print_system_information() {
    println!("System information:");
    println!("\tName:           {:?}", System::name());
    println!("\tKernel Version: {:?}", System::kernel_version());
    println!("\tOS Version:     {:?}", System::os_version());
    println!("\tHost Name:      {:?}", System::host_name());
}

fn print_metrics(sys: &System) {
    println!("Total memory: {} bytes", sys.total_memory());
    println!("Used memory: {} bytes", sys.used_memory());
    println!("Total swap: {} bytes", sys.total_swap());
    println!("Used swap: {} bytes", sys.used_swap());
}

fn print_processes(sys: &System) {
    println!("Process list:");
    for (pid, process) in sys.processes() {
        println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    }
}

