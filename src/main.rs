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
        .arg(
            Arg::new("ascii")
                .short('a')
                .long("ascii")
                .help("choose which ascii to show"),
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

    let mut ascii_type = "neocat".to_string();
    if matches.get_one::<String>("ascii").is_none() {
        ascii_type = "neocat".to_string();
    } else {
        ascii_type = matches.get_one::<String>("ascii").unwrap().to_string();
    }

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
        println!("   {}   ", ascii::get(ascii_type.clone(), 0));
        println!("   {}   ", ascii::get(ascii_type.clone(), 1));
        println!(
            "   {}   {}{}{}",
            ascii::get(ascii_type.clone(), 2),
            username.purple(),
            "@".white(),
            hostname.purple()
        );
        println!("   {}   ", ascii::get(ascii_type.clone(), 3));
        println!(
            "   {}   {} {os_name} {os_version}",
            ascii::get(ascii_type.clone(), 4),
            "OS:".bold().purple()
        );
        println!(
            "   {}   {} {host}",
            ascii::get(ascii_type.clone(), 5),
            "Host:".bold().purple()
        );
        println!(
            "   {}   {} {kernel_version}",
            ascii::get(ascii_type.clone(), 6),
            "Kernel:".bold().purple()
        );
        println!(
            "   {}   {} {uptime_hours} {} {uptime_minutes} {} {uptime_seconds} {}",
            ascii::get(ascii_type.clone(), 7),
            "Uptime:".bold().purple(),
            "hour(s),".white(),
            "minute(s),".white(),
            "second(s)".white()
        );
        println!(
            "   {}   {} {}",
            ascii::get(ascii_type.clone(), 8),
            "Desktop:".bold().purple(),
            de
        );
        println!(
            "   {}   {} {} {}{}{}",
            ascii::get(ascii_type.clone(), 9),
            "CPU:".bold().purple(),
            cpu,
            "(".white(),
            cpu_count.to_string().white(),
            ")".white()
        );
        println!(
            "   {}   {} {mem_used}{} of {mem_total}{} used",
            ascii::get(ascii_type.clone(), 10),
            "Memory:".bold().purple(),
            "mb".white(),
            "mb".white()
        );
        println!(
            "   {}   {} {swap_used}{} of {swap_total}{} used",
            ascii::get(ascii_type.clone(), 11),
            "Swap:".bold().purple(),
            "mb".white(),
            "mb".white()
        );
        println!("   {}   ", ascii::get(ascii_type.clone(), 12),);
        println!(
            "   {}   {}{}{}{}{}{}{}{}",
            ascii::get(ascii_type.clone(), 13),
            "███".black(),
            "███".red(),
            "███".yellow(),
            "███".green(),
            "███".cyan(),
            "███".blue(),
            "███".purple(),
            "███".white()
        );
        println!(
            "   {}   {}{}{}{}{}{}{}{}",
            ascii::get(ascii_type.clone(), 14),
            "███".bright_black(),
            "███".bright_red(),
            "███".bright_yellow(),
            "███".bright_green(),
            "███".bright_cyan(),
            "███".bright_blue(),
            "███".bright_purple(),
            "███".bright_white()
        );
        println!("   {}   ", ascii::get(ascii_type.clone(), 15),);
        println!("   {}   ", ascii::get(ascii_type.clone(), 16),);
        println!("   {}   ", ascii::get(ascii_type.clone(), 17),);
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
