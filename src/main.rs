use colored::Colorize;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    let mut username = "e";
    let mut hostname = "localhost";

    let mut os_name = sys.name().unwrap();
    let mut os_version = sys.os_version().unwrap();
    let mut kernel_version = sys.kernel_version().unwrap();
    let mut host = sys.host_name().unwrap();
    let mut uptime_hours = sys.uptime()/60/60;
    let mut uptime_minutes = (sys.uptime()/60)-(uptime_hours/60/60);

    // in megabytes
    let mut mem_used = sys.used_memory()/1000000;
    let mut mem_total = sys.total_memory()/1000000;
    let mut swap_used = sys.used_swap()/1000000;
    let mut swap_total = sys.total_swap()/1000000;

    println!("   ");
    println!("   {}@host", username);
    println!("   ");
    println!("   {} {os_name} {os_version}", "OS:".bold().purple());
    println!("   {} {host}", "Host:".bold().purple());
    println!("   {} {kernel_version}", "Kernel:".bold().purple());
    println!("   {} {uptime_hours} hours {uptime_minutes} minutes", "Uptime:".bold().purple());
    println!("   {} ", "Packages:".bold().purple());
    println!("   {} ", "Shell:".bold().purple());
    println!("   {} ", "DE:".bold().purple());
    println!("   {} ", "WM:".bold().purple());
    println!("   {} ", "CPU:".bold().purple());
    println!("   {} ", "GPU:".bold().purple());
    println!("   {} {mem_used}{}/{mem_total}{} used", "Memory:".bold().purple(), "MB".white(), "MB".white());
    println!("   {} {swap_used}{}/{swap_total}{} used", "Swap:".bold().purple(), "MB".white(), "MB".white());
    println!("   ");
}