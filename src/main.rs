use clap::{Arg, Command};
use colored::*;
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, System, SystemExt};
use std::fs;

fn main() {
    // Command-line argument parsing using clap
    let matches = Command::new("rufetch")
        .version("1.0")
        .author("Your Name vigneshmvgs2003@gmail.com")
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

    let ascii_art_file = "ascii_art_file.txt";
    let ascii_art = match fs::read_to_string(ascii_art_file) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read ASCII art from {}", ascii_art_file);
            return;
        }
    };    

 

    // Fetch common information
    let os_name = sys.name().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = sys
        .kernel_version()
        .unwrap_or_else(|| "Unknown".to_string());
    let hostname = sys.host_name().unwrap_or_else(|| "Unknown".to_string());
    let hours = (sys.uptime())/3600;
    let minutes = ((sys.uptime())%3600)/60;
    let seconds = (sys.uptime())% 60;

    // Print common information
    println!("{}", "rufetch".bold().blue());
    println!("{}", ascii_art );
    println!("{}: {}", "OS".green(), os_name);
    println!("{}: {}", "Kernel".green(), kernel_version);
    println!("{}: {}", "Hostname".green(), hostname);
    println!("{}: {} hours {} minutes {} seconds", "Uptime".green(), hours, minutes, seconds);

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
