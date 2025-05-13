use clap::Parser;
use std::fs;
use std::env;
use sysinfo::{
    System,
};
use include_dir::{include_dir, Dir};
use std::io::{self, Write};
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

fn main() {
    let args = Args::parse();
    let mut sys = System::new_all();
    sys.refresh_all();

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