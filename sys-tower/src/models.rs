use std::thread;
use std::time::Duration;
use sysinfo::{Disks, System};

pub const WAIT_DURATION_MS: u64 = 500;

pub struct SystemData {
    pub system_info: SystemInfo,
    pub cpu_count: usize,
    pub cpu_info: Vec<CpuInfo>,
    pub memory_info: MemoryInfo,
    pub disk_info: Vec<DiskInfo>,
    pub process_info: Vec<ProcessInfo>,
}

pub struct SystemInfo {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
}

pub struct CpuInfo {
    pub name: String,
    pub frequency: u64,
    pub vendor_id: String,
    pub brand: String,
}

pub struct MemoryInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

pub struct DiskInfo {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
}

pub struct ProcessInfo {
    pub pid: String,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

impl SystemData {
    pub fn new(sys: &mut System) -> Self {
        sys.refresh_all();
        thread::sleep(Duration::from_millis(WAIT_DURATION_MS));
        sys.refresh_all();
        thread::sleep(Duration::from_millis(WAIT_DURATION_MS));
        sys.refresh_all();

        let system_info = SystemInfo {
            name: System::name().unwrap_or("Unknown".into()),
            kernel_version: System::kernel_version().unwrap_or("Unknown".into()),
            os_version: System::os_version().unwrap_or("Unknown".into()),
            host_name: System::host_name().unwrap_or("Unknown".into()),
        };

        let cpu_count = sys.cpus().len();
        let cpu_info: Vec<CpuInfo> = sys
            .cpus()
            .iter()
            .map(|cpu| CpuInfo {
                name: cpu.name().to_string(),
                frequency: cpu.frequency(),
                vendor_id: cpu.vendor_id().to_string(),
                brand: cpu.brand().to_string(),
            })
            .collect();

        let memory_info = MemoryInfo {
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            free_memory: sys.free_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
        };

        let disk_info: Vec<DiskInfo> = Disks::new_with_refreshed_list()
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                used_space: disk.total_space() - disk.available_space(),
            })
            .collect();

        let mut processes: Vec<ProcessInfo> = sys
            .processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.to_string(),
                name: process.name().to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
            })
            .collect();

        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        processes.truncate(5);

        SystemData {
            system_info,
            cpu_count,
            cpu_info,
            memory_info,
            disk_info,
            process_info: processes,
        }
    }
}
