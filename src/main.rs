use clap::Parser;
use std::fs;
use std::env;

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show logo
    #[arg(short, long)]
    logo: String
}

fn main() {
    let args = Args::parse();
    let home_dir = env::var("HOME").expect("Unable to find home directory");
    let path = format!("{}/.config/rastfetch/{}", home_dir, args.logo);

    match fs::read_to_string(path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("error reading file: {}", e),
    }
}
