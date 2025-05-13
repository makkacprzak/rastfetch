use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::env;
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use sysinfo::{
    System,
};
use include_dir::{include_dir, Dir};
use std::io::{self, Write};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::task;

mod os_map;

const ASSETS: Dir = include_dir!("assets");

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show logo
    #[arg(short, long)]
    logo: Option<String>,

    /// Create default config
    #[arg(short, long)]
    config: bool,
}

async fn fetch_title() -> String {
    let title = "Title: Rastfetch";
    title.to_string()
}
async fn fetch_separator() -> String {
    let separator = "--------------------------------";
    separator.to_string()
}
async fn fetch_os() -> String {
    let os = System::name().unwrap_or("Can't find system name".to_string());
    let os_short =  *os_map::OS_MAP.get(&os).unwrap_or(&"unknown");
    let os_info = format!("OS: {}", os_short);
    os_info
}


#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Load the config file
    let modules = get_modules().unwrap();

    // Map of module names to their respective functions
    let mut module_functions: HashMap<&str, Arc<dyn Fn() -> Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync>> = HashMap::new();

    // Wstawianie funkcji do mapy, opakowanych w Box::pin
    module_functions.insert("title", Arc::new(|| Box::pin(fetch_title())));
    module_functions.insert("separator", Arc::new(|| Box::pin(fetch_separator())));
    module_functions.insert("os", Arc::new(|| Box::pin(fetch_os())));

    let (tx, mut rx) = mpsc::channel(modules.len());

    let mut tasks = vec![];


    for module in modules {
        if let Some(func) = module_functions.get(module.as_str()).cloned() {
            let tx_clone = tx.clone();
            let task = task::spawn(async move {
                let result = (func)().await;
                tx_clone.send(result).await.unwrap();
            });
            tasks.push(task);
        }
    }

    for _ in tasks {
        let result = rx.recv().await.unwrap();
        println!("{:?}", result);
    }

    if args.config{
        let home_dir = env::var("HOME").expect("Unable to find home directory");
        let path = format!("{}/.config/rastfetch", home_dir);
        fs::create_dir_all(path).expect("Unable to create config directory");
        io::stdout().write_all(b"Config directory created\n").unwrap();
        let config_path = format!("{}/.config/rastfetch/config.json", home_dir);
        let mut file = fs::File::create(config_path).expect("Unable to create config file");
        let config_content =
        r#"{
            "modules": [
                "title",
                "separator",
                "os"
            ]
        }"#;
        file.write_all(config_content.as_bytes()).expect("Unable to write to config file");
        io::stdout().write_all(b"Config file created\n").unwrap();
        return;
    }
    
    if let Some(logo_value) = args.logo.as_deref(){
        let home_dir = env::var("HOME").expect("Unable to find home directory");
        let path = format!("{}/.config/rastfetch/{}", home_dir, logo_value);

        match fs::read_to_string(path){
            Ok(contents) => println!("{}", contents),
            Err(e) => eprintln!("Error while reading file: {}", e),
        }
    }else{
        let os = System::name().unwrap_or("Can't find system name".to_string());
        let os_short =  *os_map::OS_MAP.get(&os).unwrap_or(&"unknown");
        let path = format!("logo/ascii/{}.txt", os_short);
        
        if let Some(file) = ASSETS.get_file(path){
            let contents = file.contents_utf8().unwrap();
            io::stdout().write_all(contents.as_bytes()).unwrap();
        }
    }

}


fn read_config() -> Result<Value, Box<dyn std::error::Error>> {
    let config_path = format!("{}/.config/rastfetch/config.json", env::var("HOME")?);
    let config_data = fs::read_to_string(config_path)?;
    let config: Value = serde_json::from_str(&config_data)?;
    Ok(config)
}

fn get_modules() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let config = read_config()?;
    if let Some(modules) = config["modules"].as_array() {
        let module_list = modules.iter()
            .filter_map(|m| m.as_str().map(|s| s.to_string()))
            .collect();
        Ok(module_list)
    } else {
        Err("Nie znaleziono klucza 'modules' lub nie jest to tablica.".into())
    }
}