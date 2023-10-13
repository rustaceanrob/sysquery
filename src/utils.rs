use std::collections::BinaryHeap;
use std::io;
use std::path::{Path, PathBuf};
use colored::Colorize;
use sysinfo::{NetworkExt, DiskExt, CpuExt, ProcessExt};
use walkdir::WalkDir;
use sysinfo::{System, SystemExt};

pub fn quicksort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quicksort(arr, 0, (len - 1) as isize);
}

fn _quicksort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        _quicksort(arr, low, p - 1);
        _quicksort(arr, p + 1, high);
    }
}

fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut i = low - 1;
    let mut j = high;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot);
    i
}

#[derive(Debug)]
pub struct ProcessInfo<'a> {
    pub name: &'a str,
    pub read_bytes: u64,
    pub written_bytes: u64,
    pub memory: u64,
    pub status: String,
}

impl Eq for ProcessInfo<'_> {}

impl PartialEq for ProcessInfo<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.memory.eq(&other.memory)
    }
}

impl Ord for ProcessInfo<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.memory.cmp(&other.memory).reverse()
    }
}

impl PartialOrd for ProcessInfo<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
}

impl Eq for FileInfo {}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.size.eq(&other.size)
    }
}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size.cmp(&other.size).reverse()
    }
}

impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_largest_files(start_dir: &Path, n: u8) -> io::Result<Vec<FileInfo>> {
    let mut heap = BinaryHeap::new();

    for entry in WalkDir::new(start_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let size = entry.metadata()?.len();
            heap.push(FileInfo {
                path: entry.path().to_path_buf(),
                size,
            });
            // how?
            if heap.len() > n.into() {
                heap.pop();
            }
        }
    }

    let mut vec = heap.into_vec();
    quicksort(&mut vec);
    // vec.truncate(n.into());
    Ok(vec)

}

pub fn digest() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("\nHere is your system digest: \n");
    println!("Core system information:");
    let available_memory = format!("{:.2}", system.available_memory() as f64 / 1_073_741_824 as f64).bright_green();
    println!("Memory available: {} gigabytes", available_memory);
    let free_memory = format!("{:.2}", system.free_memory() as f64 / 1_073_741_824 as f64).bright_green();
    println!("Unallocated memory available: {} gigabytes", free_memory);
    let total_memory = format!("{:.2}", system.total_memory() as f64 / 1_073_741_824 as f64).bright_green();
    println!("Total memory available: {} gigabytes", total_memory);

    match system.physical_core_count() {
        Some(os) => {
            let c_os = os.to_string().bright_green();
            println!("Number of physical cores: {}", c_os);
        },
        None => {
            println!("Physical core count unavailable");
        },
    }

    for disk in system.disks() {
        let space = format!("{:.2}", disk.available_space() as f64 / 1_073_741_824 as f64).bright_green();
        println!("Disk space available: {} gigabytes", space);
        break; //only return the OS disk
    }
    let bt = (system.boot_time() / 60 / 60 / 60 / 24).to_string().bright_green();
    println!("Days since first boot: {}", bt);

    println!("\nCPU usage:");
    for cpu in system.cpus() {
        println!("{}: usage {}; Vendor ID: {}", cpu.brand(), format!("{:.2}%", cpu.cpu_usage()).bright_green(), cpu.vendor_id());
    }
    

    println!("\nOperating system information:");
    println!("Distribution ID: {:?}", system.distribution_id());
    match system.name() {
        Some(os) => {
            let c_os = os.bright_green();
            println!("OS Name: {}", c_os);
        },
        None => {
            println!("OS name unavailable");
        },
    }
    match system.long_os_version() {
        Some(os) => {
            let c_os = os.bright_green();
            println!("Operating system version: {}", c_os);
        },
        None => {
            println!("OS system version unavailable");
        },
    }
}

pub fn network() -> () {
    let mut system = System::new_all();
    system.refresh_all();

    println!("\nNetwork I/O:\n");
    let networks = system.networks();
    for (interface_name, data) in networks {
        println!(
            "[{}]; MAC address: {}; total packets in: {}; total packets out: {}",
            interface_name.bright_green(),
            data.mac_address().to_string().bright_yellow(),
            data.total_packets_received().to_string().bright_green(),
            data.total_packets_transmitted().to_string().bright_blue(),
        );
        println!(
            "total errors on packets in: {}; total errors packets out: {}\n",
            data.total_errors_on_received().to_string().bright_red(),
            data.total_errors_on_transmitted().to_string().bright_red(),
        );
    }
}

pub fn processes(n: u8) -> () {
    let mut system = System::new_all();
    system.refresh_all();
    let mut heap = BinaryHeap::new();

    for (_, process) in system.processes() {
        heap.push(ProcessInfo {
            name: process.name(),
            read_bytes: process.disk_usage().total_read_bytes,
            written_bytes: process.disk_usage().total_written_bytes,
            memory: process.memory(),
            status: process.status().to_string(),

        });
    }

    let mut vec = heap.into_vec();
    quicksort(&mut vec);
    vec.truncate(n.into());

    println!("\nCurrent processes: \n");
    for p in vec {
        let name = p.name.bright_green();
        let memory = format!("{:.2}", p.memory as f64 / 1_073_741_824 as f64).to_string().bright_green();
        let read = format!("{:.2}", p.read_bytes as f64 / 1_073_741_824 as f64).to_string().bright_green();
        let written = format!("{:.2}", p.written_bytes as f64 / 1_073_741_824 as f64).to_string().bright_green();
        println!("[{}]: memory usage: {} gigabytes", name, memory);
        println!("[{}]: disk usage: total disk reads: {} gigabytes; total disk writes: {} gigabytes;", name, read, written);
        println!("[{}]: status: {} \n", name, p.status.bright_green());
    }

}