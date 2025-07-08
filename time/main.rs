use sysinfo::System;
use os_info;
use whoami;
use colored::*;
use std::env;
use std::process::Command;

// Helper: Get kernel version via `uname -r`
fn get_kernel_version() -> String {
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }
    "Unknown".to_string()
}

fn main() {
    let sys = System::new_all();

    // Gather system info
    let os = os_info::get();
    let username = whoami::username();
    let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "unknown-host".into());
    let shell = env::var("SHELL").unwrap_or_else(|_| "unknown".into());
    let distro = os.to_string();
    let kernel = get_kernel_version();
    let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let uptime_display = format!("{:02}h {:02}m", hours, minutes);
    let cpu = sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_else(|| "Unknown".into());
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    // GitHub ASCII logo
    let logo = vec![
        r"      MMM.           .MMM",
        r"      MMMMMMMMMMMMMMMMMMM",
        r"      MMMMMMMMMMMMMMMMMMM",
        r"      MMMMMMMMMMMMMMMMMMM",
        r"      MMMM::- -:::::::-::",
        r"      MMMM  :   '::'  :  ",
        r"      MMMM .:         :. ",
        r"      MMMM .:         :. ",
        r"      MMMM  ':.___.:'  ",
        r"      MMMM    '---'    ",
        r"   .::::::::::..        ",
        r"  :::::::::::::::  .:::. ",
        r" :::::::::::::::::..:::  ",
        r" ::::::::::::::::::::'   ",
        r"  ':::::::::::::::'      ",
        r"    `'::::::::'`         ",
    ];

    // Display logo + aligned info
    for (i, line) in logo.iter().enumerate() {
        let info = match i {
            2 => format!("{}@{}", username.green(), hostname.yellow()),
            3 => format!("OS: {}", distro.cyan()),
            4 => format!("Kernel: {}", kernel.purple()),
            5 => format!("Uptime: {}", uptime_display),
            6 => format!("Shell: {}", shell),
            7 => format!("CPU: {}", cpu),
            8 => format!("Memory: {}/{} MB", used_memory / 1024, total_memory / 1024),
            _ => String::new(),
        };
        println!("{}   {}", line.bright_black(), info);
    }
}
