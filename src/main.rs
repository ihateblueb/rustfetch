use clap::{Arg, Command};
use std::env;

use colored::Colorize;
use sysinfo::{CpuExt, System, SystemExt};

mod ascii;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("start")
                .short('s')
                .long("start")
                .num_args(0)
                .help("show start output instead"),
        )
        .get_matches();

    let sys = System::new_all();

    let mut mode = "default";
    if matches.get_flag("start") {
        mode = "start";
    } else {
        mode = "default";
    }

    let username = whoami::username();
    let hostname = whoami::hostname();

    let os_name = sys.name().unwrap();
    let os_version = sys.os_version().unwrap();
    let kernel_version = sys.kernel_version().unwrap();
    let host = sys.host_name().unwrap();
    let uptime_hours = sys.uptime() / 60 / 60;
    let uptime_minutes =
        (sys.uptime() / 60) - (str::parse::<u64>(uptime_hours.to_string().as_str()).unwrap() * 60);
    let uptime_seconds = (sys.uptime())
        - (str::parse::<u64>(uptime_hours.to_string().as_str()).unwrap() * 60 * 60)
        - (str::parse::<u64>(uptime_minutes.to_string().as_str()).unwrap() * 60);

    let mut de = whoami::desktop_env().to_string().replace("Unknown: ", "");

    if de == "" {
        de = "Unknown".to_string();
    } else if de == " " {
        de = "Unknown".to_string();
    }

    // in megabytes
    let mem_used = sys.used_memory() / 1000000;
    let mem_total = sys.total_memory() / 1000000;
    let swap_used = sys.used_swap() / 1000000;
    let swap_total = sys.total_swap() / 1000000;

    let cpu = sys.global_cpu_info().brand();
    let cpu_count = sys.cpus().len();

    if mode == "default" {
        println!("   ");
        println!(
            "   {}{}{}",
            username.purple(),
            "@".white(),
            hostname.purple()
        );
        println!("   ");
        println!("   {} {os_name} {os_version}", "OS:".bold().purple());
        println!("   {} {host}", "Host:".bold().purple());
        println!("   {} {kernel_version}", "Kernel:".bold().purple());
        println!(
            "   {} {uptime_hours} {} {uptime_minutes} {} {uptime_seconds} {}",
            "Uptime:".bold().purple(),
            "hour(s),".white(),
            "minute(s),".white(),
            "second(s)".white()
        );
        println!("   {} {}", "Desktop:".bold().purple(), de);
        println!(
            "   {} {} {}{}{}",
            "CPU:".bold().purple(),
            cpu,
            "(".white(),
            cpu_count.to_string().white(),
            ")".white()
        );
        println!(
            "   {} {mem_used}{} of {mem_total}{} used",
            "Memory:".bold().purple(),
            "mb".white(),
            "mb".white()
        );
        println!(
            "   {} {swap_used}{} of {swap_total}{} used",
            "Swap:".bold().purple(),
            "mb".white(),
            "mb".white()
        );
        println!("   ");
    } else if mode == "start" {
        println!("  ");
        println!(
            "   {}{}{}",
            "Welcome back, ".purple(),
            username.purple(),
            "!".purple()
        );
        println!("  ");
        println!(
            "   {} {uptime_hours} {} {uptime_minutes} {} {uptime_seconds} {}",
            "Uptime:".bold().purple(),
            "hour(s),".white(),
            "minute(s),".white(),
            "second(s)".white()
        );
        println!(
            "   {} {mem_used}{} of {mem_total}{} used",
            "Memory:".bold().purple(),
            "mb".white(),
            "mb".white()
        );
        println!(
            "   {} {swap_used}{} of {swap_total}{} used",
            "Swap:".bold().purple(),
            "mb".white(),
            "mb".white()
        );
        println!("  ");
    }
}
