//! This module contains all the asynchronous functions for fetching information, and a HashMap binding module names to these functions
use sysinfo::{
    CpuRefreshKind, Disk, Disks, MemoryRefreshKind, RefreshKind, System
};
use std::collections::HashMap;
use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use std::path::Path;
use whoami;

use crate::{os_map, ASSETS};

type ModuleFunction = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync>;

/// Hash map mapping functions for fetching information asynchronously to strings present in config file
pub fn get_module_functions() -> HashMap<&'static str, ModuleFunction> {
        // Map of module names to their respective functions
    let mut module_functions: HashMap<&str, Arc<dyn Fn() -> Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync>> = HashMap::new();

    // Wstawianie funkcji do mapy, opakowanych w Box::pin
    module_functions.insert("title", Arc::new(|| Box::pin(fetch_title())));
    module_functions.insert("separator", Arc::new(|| Box::pin(fetch_separator())));
    module_functions.insert("os", Arc::new(|| Box::pin(fetch_os())));
    module_functions.insert("kernel", Arc::new(|| Box::pin(fetch_kernel())));
    module_functions.insert("uptime", Arc::new(|| Box::pin(fetch_uptime())));
    module_functions.insert("memory", Arc::new(|| Box::pin(fetch_memory())));
    module_functions.insert("shell", Arc::new(|| Box::pin(fetch_shell())));
    module_functions.insert("cpu", Arc::new(|| Box::pin(fetch_cpu())));
    module_functions.insert("cpu_usage", Arc::new(|| Box::pin(fetch_cpu_usage())));
    module_functions.insert("swap", Arc::new(|| Box::pin(fetch_swap())));
    module_functions.insert("disks", Arc::new(|| Box::pin(fetch_disks())));
    module_functions.insert("terminal", Arc::new(|| Box::pin(fetch_terminal_emulator())));
    module_functions.insert("colors", Arc::new(|| Box::pin(fetch_color_palette())));
    module_functions.insert("bios", Arc::new(|| Box::pin(fetch_bios())));
    module_functions.insert("editor", Arc::new(|| Box::pin(fetch_editor())));
    module_functions.insert("platform", Arc::new(|| Box::pin(fetch_platform())));
    module_functions.insert("chassis", Arc::new(|| Box::pin(fetch_chassis())));


    module_functions
}

/// Title is in (username)@(hostname) format
/// Data fetched from environment variables
async fn fetch_title() -> String {
    let user = env::var("USER");
    let hostname = env::var("HOSTNAME");
    let title = format!("\n$1{}$2@$1{}$2", user.unwrap_or(format!("{}", "Unknown")), hostname.unwrap_or(format!("{}", "Unknown")));
    title
}
/// Just prints a separator
async fn fetch_separator() -> String {
    let separator = "$2-----------$1";
    separator.to_string()
}
/// Fetches disto in a pretty format using whoami
async fn fetch_os() -> String {
    let os = whoami::distro();
    let os_string = format!("$3OS:$2 {}", os);
    os_string
}
/// Fetches kernel name and version from /proc/version
async fn fetch_kernel() -> String {
    let kernel_string: String = match fs::read_to_string("/proc/version") {
        Ok(content) => {
            let kernel = {
                let parts: Vec<&str> = content.split_whitespace().collect();
                (parts[0].to_string(), parts[2].to_string())
            };
            format!("$3Kernel:$2 {} v{}", kernel.0, kernel.1)
        }
        Err(_) => {
            let kernel_version = System::kernel_long_version();
            format!("$3Kernel:$2 {}", kernel_version)
        }
    };
    kernel_string
}
/// Fetches uptime using sysinfo
async fn fetch_uptime() -> String {
    let uptime = System::uptime();
    let uptime_string = format!("$3Uptime:$2 {}", format_time(uptime));
    uptime_string
}
/// Formats uptime in a pretty way
fn format_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let seconds_left_after_hours = seconds % 3600;
    let minutes = seconds_left_after_hours / 60;
    let seconds = seconds_left_after_hours % 60;

    let mut parts = Vec::new();

    if hours > 0 {
        parts.push(format!("$2{}$3h$2", hours));
    }
    if minutes > 0 || hours > 0 {
        parts.push(format!("$2{}$3m$2", minutes));
    }
    parts.push(format!("$2{}$3s$2", seconds));

    parts.join(" ")
}
/// Fetches used_memory/total_memory - (used)% using sysinfo
async fn fetch_memory() -> String {
    let sys = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
    );
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let used_percentage = (used_memory as f64 / total_memory as f64) * 100.0;
    let memory_string = format!(
        "$3Memory Used: $2{:.2} $3GiB / $2{:.2} $3GiB $4({}%)$2",
        used_memory as f64 / 1073741824.0,
        total_memory as f64 / 1073741824.0,
        used_percentage as u8
    );
    memory_string
}
/// Fetches shell from the SHELL env variable
async fn fetch_shell() -> String {
    if let Ok(shell) = std::env::var("SHELL") {
        let shell_name = shell.split('/').last().unwrap_or("Unknown");
        return format!("$3Shell:$2 {}", shell_name)
    } else {
        "$3Shell:$2 Unknown".to_string()
    }
}

/* This function works great, if you don't care about speed.
    * fething shell version via command line is very slow, so for now no version
async fn fetch_shell() -> String {
    if let Ok(shell) = std::env::var("SHELL") {
        let shell_name = shell.split('/').last().unwrap_or("Unknown");
        let shell_command = os_map::SHELL_VERSIONS.get(shell_name).unwrap_or(&"");
        if shell_command.is_empty() {
            return format!("$3Shell:$2 {}", shell_name)
        }
        match Command::new(&shell)
            .arg("-c")
            .arg(shell_command)
            .output()
        {
            Ok(output) => {
                let shell_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return format!("$3Shell:$2 {} {}", shell_name, shell_version);
            }
            Err(_) => return format!("$3Shell:$2 {} Unknown", shell_name),
        }
    } else {
        "$3Shell:$2 Unknown".to_string()
    }
}
*/
/// Fetches cpu information using sysinfo
async fn fetch_cpu() -> String {
    let sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
    );
    
    let cpus = sys.cpus();
    let cores = cpus.len();
    let cpu = &cpus[0];
    let mut buffer = String::new();
    let brand = cpu.brand();
    let name = cpu.name();
    let freq = cpu.frequency();
    let final_str = format!("$3CPU: $2{} {} {}$3core $2{}$3Hz$2", brand, name, cores, freq);
    buffer.push_str(&final_str);
    buffer
}

/// For now this function is not used, because it is slow.
/// Fetches CPU usage in percentage using sysinfo
async fn fetch_cpu_usage() -> String {
    let mut s = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );
    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPUs again to get actual value.
    s.refresh_cpu_all();
    let cpus = s.cpus();
    let cpu = &cpus[0];
    let cpu_usage = cpu.cpu_usage();
    format!( "$3CPU Usage: $4{:.2}%$2", cpu_usage)
}
/// Fetches used_swap/total_swap - (used)%
async fn fetch_swap() -> String {
    let sys = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything())
    );
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let swap_percentage = (used_swap as f64 / total_swap as f64) * 100.0;
    let swap_string = format!(
        "$3Swap Used: $2{:.2} $3GiB / $2{:.2} $3GiB $4({}%)$2",
        used_swap as f64 / 1073741824.0,
        total_swap as f64 / 1073741824.0,
        swap_percentage as u8
    );
    swap_string
}
/// Fetches disk information using sysinfo and/or /proc/mounts
async fn fetch_disks() -> String {
    let disks = Disks::new_with_refreshed_list();
    let mut buffer = String::new();
    let mut selected_disks = vec![];
    let mut root_btrfs_disk = None;
    let mut min_subvolid_disk = None;
    let mut min_subvolid = u32::MAX;
    let mut mounts = None;

    for disk in disks.list() {
        let file_system = disk.file_system().to_string_lossy();
        let mount_point = disk.mount_point().to_string_lossy();
        if file_system == "vfat" || file_system == "overlay" || mount_point == "/boot" {
            continue;
        }

        if file_system == "btrfs" {
            if mount_point == "/" {
                root_btrfs_disk = Some(disk);
                break;
            }

            // Lazily read /proc/mounts only if we actually encounter a btrfs disk
            if mounts.is_none() {
                mounts = match File::open("/proc/mounts") {
                    Ok(file) => Some(BufReader::new(file)
                        .lines()
                        .filter_map(Result::ok)
                        .collect::<Vec<String>>()),
                    Err(_) => None,
                };
            }

            if let Some(mount_lines) = &mounts {
                for line in mount_lines {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 2 && parts[1] == mount_point && parts[2] == "btrfs" {
                        if line.contains("subvolid=5") {
                            root_btrfs_disk = Some(disk);
                            break;
                        }
                        if let Some(subvolid) = parts.iter().find(|&&part| part.starts_with("subvolid=")) {
                            let subvolid_value = subvolid[9..].parse().unwrap_or(u32::MAX);
                            if subvolid_value < min_subvolid {
                                min_subvolid = subvolid_value;
                                min_subvolid_disk = Some(disk);
                            }
                        }
                    }
                }
            }
            continue;
        }
        selected_disks.push(disk);
    }

    // Prefer root btrfs disk, fall back to the one with the smallest subvolid
    if let Some(disk) = root_btrfs_disk.or(min_subvolid_disk) {
        buffer.push_str(&format_disk_info(disk));
    } else if !selected_disks.is_empty() {
        for disk in selected_disks {
            buffer.push_str(&format_disk_info(disk));
        }
    } else {
        buffer.push_str("$3Disk: $2Unknown\n");
    }

    buffer
}

/// Formats disk information
fn format_disk_info(disk: &Disk) -> String {
    let file_system = disk.file_system().to_string_lossy();
    let mount_point = disk.mount_point().to_string_lossy();
    let size = disk.total_space();
    let used = size - disk.available_space();
    let used_percentage = (used as f64 / size as f64) * 100.0;
    format!("$3Disk ({}): $2{:.2} $3GiB / $2{:.2} $3GiB $4({}%)$2 - {}\n",
        mount_point,
        used as f64 / 1073741824.0,
        size as f64 / 1073741824.0,
        used_percentage as u8,
        file_system
    )
}
/// Checks what terminal you're using by trying environment variables associated with common terminal emulators
/// If no matches are found, prints the terminals framework
async fn fetch_terminal_emulator() -> String {
    // Check common emulators
    let mut buffer = "$3Terminal: $2".to_string();
    if let Ok(_) = env::var("ALACRITTY_LOG") {
        buffer.push_str("alacritty");
        return buffer;
    }
    if let Ok(_) = env::var("TERMINATOR_UUID") {
        buffer.push_str("terminator");
        return buffer;
    }
    if let Ok(_) = env::var("VTE_VERSION") {
        buffer.push_str("gnome-terminal");
        return buffer;
    }
    if let Ok(_) = env::var("KONSOLE_PROFILE_NAME") {
        buffer.push_str("kosnole");
        return buffer;
    }
    if let Ok(term_program) = env::var("TERM_PROGRAM") {
        match term_program.as_str() {
            "Apple_Terminal" => buffer.push_str("terminal"),
            "iTerm.app" => buffer.push_str("iTerm"),
            "Hyper" => buffer.push_str("hyper"),
            "Kitty" => buffer.push_str("kitty"),
            "vscode" => buffer.push_str("vscode"),
            _ => buffer.push_str(&term_program),
        }
    }
    // If nothing found check terminal framework
    if let Ok(term) = env::var("TERM") {
        match term.as_str() {
            "xterm-kitty" => buffer.push_str("kitty"),
            "tmux-256color" => buffer.push_str("tmux"),
            "screen-256color" => buffer.push_str("screen"),
            "linux" => buffer.push_str("tty"),
            _ => (),
        }
    }
    buffer
}
/// Fetches color palette file from embeded "assets" directory
async fn fetch_color_palette() -> String {
            let path = format!("ansi/palette.ansi.txt");
            if let Some(file) = ASSETS.get_file(path) {
                let contents = file.contents_utf8().unwrap();
                contents.to_string()
            } else {
                "Palette file not found".to_string()
            }
}
/// Fetches bios information from /sys/class/dmi/id/ and /sys/firmware/efi/ directories
async fn fetch_bios() -> String {
    let bios_version = fs::read_to_string("/sys/class/dmi/id/bios_version")
        .unwrap_or_else(|_| "Unknown".to_string());
    let bios_release =  fs::read_to_string("/sys/class/dmi/id/bios_release")
        .unwrap_or_else(|_| "Unknown".to_string());
    let bios_version = bios_version.trim();
    let bios_release = bios_release.trim();
    let path = Path::new("/sys/firmware/efi");
    let bios_type = match path.exists() {
        true => "UEFI",
        false => "Legacy",
    }; 
    format!("$3BIOS ({}): $2{} {}$1",bios_type ,bios_version, bios_release)
}  
/// Fetches default editor from env variable
async fn fetch_editor() -> String {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "Unknown".to_string());
    let editor_name = editor.split('/').last().unwrap_or("Unknown");
    format!("$3Editor:$2 {}", editor_name)
}

async fn fetch_platform() -> String {
    let platform = whoami::platform();
    format!("$3Platform: $2{}", platform)
}

async fn fetch_chassis() -> String {
    let chassis_code = fs::read_to_string("/sys/class/dmi/id/chassis_type").unwrap_or("Unknown".to_string());
    let code_trimmed = chassis_code.trim();
    let chassis_type = *os_map::CHASSIS_TYPES.get(code_trimmed).unwrap_or(&"Unknown chassis code");
    format!("$3Chassis: $2{}", chassis_type)
}