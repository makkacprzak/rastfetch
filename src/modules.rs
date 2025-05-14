use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, RefreshKind, System, Disks
};
use std::collections::HashMap;
use std::env;
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use whoami;

use crate::ASSETS;

type ModuleFunction = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync>;

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
    module_functions.insert("swap", Arc::new(|| Box::pin(fetch_swap())));
    module_functions.insert("disks", Arc::new(|| Box::pin(fetch_disks())));
    module_functions.insert("terminal", Arc::new(|| Box::pin(fetch_terminal_emulator())));
    module_functions.insert("colors", Arc::new(|| Box::pin(fetch_color_palette())));    


    module_functions
}

async fn fetch_title() -> String {
    let user = whoami::username();
    let hostname = whoami::devicename();
    let title = format!("$1{}$2@$1{}$2", user, hostname);
    title
}
async fn fetch_separator() -> String {
    let separator = "$2-----------$1";
    separator.to_string()
}
async fn fetch_os() -> String {
    let os = System::name().unwrap_or("Can't find system name".to_string());
    let os_string = format!("$3OS:$2 {}", os);
    os_string
}
async fn fetch_kernel() -> String {
    let kernel = System::kernel_version().unwrap_or("Can't find kernel version".to_string());
    let kernel_string = format!("$3Kernel:$2 {}", kernel);
    kernel_string
}
async fn fetch_uptime() -> String {
    let uptime = System::uptime();
    let uptime_string = format!("$3Uptime:$2 {}", format_time(uptime));
    uptime_string
}
async fn fetch_memory() -> String {
    let sys = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
    );
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let used_percentage = (used_memory as f64 / total_memory as f64) * 100.0;
    let memory_string = format!(
        "$3Memory Used: $2{:.2} $3GB / $2{:.2} $3GB $4({}%)$2",
        used_memory as f64 / 1073741824.0,
        total_memory as f64 / 1073741824.0,
        used_percentage as u8
    );
    memory_string
}

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

async fn fetch_swap() -> String {
    let sys = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything())
    );
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let swap_percentage = (used_swap as f64 / total_swap as f64) * 100.0;
    let swap_string = format!(
        "$3Swap Used: $2{:.2} $3GB / $2{:.2} $3GB $4({}%)$2",
        used_swap as f64 / 1073741824.0,
        total_swap as f64 / 1073741824.0,
        swap_percentage as u8
    );
    swap_string
}

async fn  fetch_disks() -> String {
    let disks = Disks::new_with_refreshed_list();
    let mut buffer = String::new();
    for disk in disks.list() {
        let mount_point = disk.mount_point();
        let size = disk.total_space();
        let used = size - disk.available_space();
        let file_system = disk.file_system();
        let used_percentage = (used as f64 / size as f64) * 100.0;
        let final_str = format!("$3Disk ({}): $2{:.2} $3GB / $2{:.2} $3GB $4({}%)$2 - {}\n",
            mount_point.to_string_lossy(),
            used as f64 / 1073741824.0,
            size as f64 / 1073741824.0,
            used_percentage as u8,
            file_system.to_string_lossy()
        );
        buffer.push_str(&final_str);
    }
    buffer
}


async fn fetch_terminal_emulator() -> String {
    // Najpierw sprawdzamy bardziej jednoznaczne zmienne
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
    if let Ok(test_program) = env::var("TERM_PROGRAM") {
        match test_program.as_str() {
            "Apple_Terminal" => buffer.push_str("terminal"),
            "iTerm.app" => buffer.push_str("iTerm"),
            "Hyper" => buffer.push_str("hyper"),
            "Kitty" => buffer.push_str("kitty"),
            "vscode" => buffer.push_str("vscode"),
            _ => (),
        }
    }
    // Jeśli nic nie znaleziono, sprawdzamy bardziej ogólne zmienne
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

async fn fetch_color_palette() -> String {
            let path = format!("ansi/palette.ansi.txt");
            if let Some(file) = ASSETS.get_file(path) {
                let contents = file.contents_utf8().unwrap();
                contents.to_string()
            } else {
                "Palette file not found".to_string()
            }
}

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