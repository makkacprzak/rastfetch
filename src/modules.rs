use sysinfo::{
    System, MemoryRefreshKind, RefreshKind
};
use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;
use std::process::Command;
use std::sync::Arc;
use whoami;

use crate::os_map;

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
    module_functions.insert("shell", Arc::new(|| Box::pin(get_shell())));

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
    let os_string = format!("$1OS:$2 {}", os);
    os_string
}
async fn fetch_kernel() -> String {
    let kernel = System::kernel_version().unwrap_or("Can't find kernel version".to_string());
    let kernel_string = format!("$1Kernel:$2 {}", kernel);
    kernel_string
}
async fn fetch_uptime() -> String {
    let uptime = System::uptime();
    let uptime_string = format!("$1Uptime:$2 {}", format_time(uptime));
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
        "$1Memory Used: $2{:.2} $1MB / $2{:.2} $1MB $2({}$1%$2)",
        used_memory as f64 / 1073741824.0,
        total_memory as f64 / 1073741824.0,
        used_percentage as u8
    );
    memory_string
}

async fn get_shell() -> String {
    if let Ok(shell) = std::env::var("SHELL") {
        let shell_name = shell.split('/').last().unwrap_or("Unknown");
        let shell_command = os_map::SHELL_VERSIONS.get(shell_name).unwrap_or(&"");
        if shell_command.is_empty() {
            return format!("$1Shell:$2 {}", shell_name)
        }
        match Command::new(&shell)
            .arg("-c")
            .arg(shell_command)
            .output()
        {
            Ok(output) => {
                let shell_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return format!("$1Shell:$2 {} {}", shell_name, shell_version);
            }
            Err(_) => return format!("$1Shell:$2 {} Unknown", shell_name),
        }
    } else {
        "$1Shell:$2 Unknown".to_string()
    }
}

fn format_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let seconds_left_after_hours = seconds % 3600;
    let minutes = seconds_left_after_hours / 60;
    let seconds = seconds_left_after_hours % 60;

    let mut parts = Vec::new();

    if hours > 0 {
        parts.push(format!("$2{}$1h$2", hours));
    }
    if minutes > 0 || hours > 0 {
        parts.push(format!("$2{}$1m$2", minutes));
    }
    parts.push(format!("$2{}$1s$2", seconds));

    parts.join(" ")
}