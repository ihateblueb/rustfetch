use colored::Colorize;
use sysinfo::{System, SystemExt, CpuExt};

fn main() {
    let mut sys = System::new_all();

    let mut username = whoami::username();
    let mut hostname = whoami::hostname();

    let mut os_name = sys.name().unwrap();
    let mut os_version = sys.os_version().unwrap();
    let mut kernel_version = sys.kernel_version().unwrap();
    let mut host = sys.host_name().unwrap();
    let mut uptime_hours = sys.uptime()/60/60;
    let mut uptime_minutes = (sys.uptime()/60)-(str::parse::<u64>(uptime_hours.to_string().as_str()).unwrap()*60);

    let mut de = whoami::desktop_env().to_string().replace("Unknown: ", "");

    if de == "" {
        de = "Unknown".to_string();
    } else if de == " " {
        de = "Unknown".to_string();
    }

    // in megabytes
    let mut mem_used = sys.used_memory()/1000000;
    let mut mem_total = sys.total_memory()/1000000;
    let mut swap_used = sys.used_swap()/1000000;
    let mut swap_total = sys.total_swap()/1000000;

    let mut cpu = sys.global_cpu_info().brand();
    let mut cpu_count = sys.cpus().len();

    println!("   ");
    println!("   {}{}{}", username.purple(), "@".white(), hostname.purple());
    println!("   ");
    println!("   {} {os_name} {os_version}", "OS:".bold().purple());
    println!("   {} {host}", "Host:".bold().purple());
    println!("   {} {kernel_version}", "Kernel:".bold().purple());
    println!("   {} {uptime_hours} {} {uptime_minutes} {}", "Uptime:".bold().purple(), "hours,".white(), "minutes".white());
    println!("   {} {}", "Desktop:".bold().purple(), de);
    println!("   {} {} {}{}{}", "CPU:".bold().purple(), cpu, "(".white(), cpu_count.to_string().white(), ")".white()); 
    println!("   {} {mem_used}{}/{mem_total}{} used", "Memory:".bold().purple(), "MB".white(), "MB".white());
    println!("   {} {swap_used}{}/{swap_total}{} used", "Swap:".bold().purple(), "MB".white(), "MB".white());
    println!("   ");
}