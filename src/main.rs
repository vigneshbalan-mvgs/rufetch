use clap::{Arg, Command};
use colored::*;
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, System, SystemExt};

fn main() {
    // Command-line argument parsing using clap
    let matches = Command::new("rufetch")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A fast system information fetcher written in Rust")
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Display all system information"),
        )
        .arg(
            Arg::new("cpu")
                .short('c')
                .long("cpu")
                .help("Display CPU information"),
        )
        .arg(
            Arg::new("memory")
                .short('m')
                .long("memory")
                .help("Display memory information"),
        )
        .arg(
            Arg::new("disk")
                .short('d')
                .long("disk")
                .help("Display disk information"),
        )
        .arg(
            Arg::new("network")
                .short('n')
                .long("network")
                .help("Display network information"),
        )
        .get_matches();

    // Initialize the system struct
    let mut sys = System::new_all();
    sys.refresh_all();

    // Fetch common information
    let os_name = sys.name().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = sys
        .kernel_version()
        .unwrap_or_else(|| "Unknown".to_string());
    let hostname = sys.host_name().unwrap_or_else(|| "Unknown".to_string());
    let uptime = sys.uptime();

    // Print common information
    println!("{}", "rufetch".bold().blue());
    println!("{}: {}", "OS".green(), os_name);
    println!("{}: {}", "Kernel".green(), kernel_version);
    println!("{}: {}", "Hostname".green(), hostname);
    println!("{}: {} seconds", "Uptime".green(), uptime);

    // Display information based on user input
    if matches.contains_id("all") || matches.contains_id("cpu") {
        let cpu_brand = sys.global_processor_info().brand().to_string();
        let cpu_usage = sys.global_processor_info().cpu_usage();
        println!("{}: {}", "CPU".green(), cpu_brand);
        println!("{}: {:.2}%", "CPU Usage".green(), cpu_usage);
    }

    if matches.contains_id("all") || matches.contains_id("memory") {
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        println!("{}: {} KB", "Total Memory".green(), total_memory);
        println!("{}: {} KB", "Used Memory".green(), used_memory);
    }

    if matches.contains_id("all") || matches.contains_id("disk") {
        for disk in sys.disks() {
            println!(
                "{}: {} ({} bytes free)",
                "Disk".green(),
                disk.name().to_string_lossy(),
                disk.available_space()
            );
        }
    }

    if matches.contains_id("all") || matches.contains_id("network") {
        for (interface_name, data) in sys.networks() {
            println!(
                "{}: {} - {} bytes received, {} bytes transmitted",
                "Network".green(),
                interface_name,
                data.received(),
                data.transmitted()
            );
        }
    }
}
