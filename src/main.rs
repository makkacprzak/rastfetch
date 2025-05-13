use clap::Parser;
use std::fs;
use std::env;
use sysinfo::{
    System
};
use include_dir::{include_dir, Dir};
use std::io::{self, Write};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::task;
use strip_ansi_escapes::strip_str;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod os_map;
mod modules;

const ASSETS: Dir = include_dir!("assets");

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



#[tokio::main]
async fn main() {
    let args = Args::parse();


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
    


    // Load the config file
    let modules = get_modules().unwrap();

    let module_functions = modules::get_module_functions();

    let (tx, mut rx) = mpsc::channel(modules.len());

    let mut tasks = vec![];

    // Dla każdego modułu tworzysz zadanie, wysyłasz wynik przez kanał z indeksem
    for (index, module) in modules.iter().enumerate() {
        if let Some(func) = module_functions.get(module.as_str()).cloned() {
            let tx_clone = tx.clone();
            let task = task::spawn(async move {
                let result = (func)().await;
                // Wyślij wynik wraz z indeksem
                tx_clone.send((index, result)).await.unwrap();
            });
            tasks.push(task);
        }
    }

    let logo = read_logo(args.logo);
    let logo_lines: Vec<String> = logo.lines().map(|line| line.to_string()).collect();

    let mut max_width = 0;
    for line in &logo_lines {
        let stripped_line = strip_str(line);
        if count_chars_without_markers(&stripped_line) > max_width {
            max_width = stripped_line.len();
        }
    }

    // Odbierasz wyniki, umieszczając je w odpowiednich miejscach w wektorze
    let mut results = vec![String::new(); modules.len()];
    for _ in tasks {
        let (index, result) = rx.recv().await.unwrap();
        results[index] = result;
    }

    let output_lines = format_terminal_output(&logo_lines, &results, max_width);
    let os = System::name().unwrap_or("Can't find system name".to_string());
    let binding = &[Color::White];
    let os_color = *os_map::OS_COLORS.get(os.as_str()).unwrap_or(&(binding as &[Color]));
    for line in output_lines {
        print_colored(&line, os_color.to_vec()).unwrap();
    }

}

fn count_chars_without_markers(text: &str) -> usize {
    let mut count = 0;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' {
            if let Some(&next_char) = chars.peek() {
                if next_char.is_ascii_digit() {
                    chars.next(); // pomijamy cyfrę po $
                    continue;
                }
            }
        }
        count += 1;
    }
    count
}

fn format_terminal_output(logo_lines: &[String], results: &[String], img_width: usize) -> Vec<String> {
    let longer_length = results.len().max(logo_lines.len());
    let mut final_vector = Vec::new();

    for i in 0..longer_length {
        let mut scalony_string = String::new();

        let logo_line = logo_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        let result_line = results.get(i).map(|s| s.as_str()).unwrap_or("");

        let mut prefix = " ".repeat(img_width);

        // Wstaw logo_line na początek prefixu (lub jego część)
        if !logo_line.is_empty() {
            prefix.replace_range(0..logo_line.len().min(img_width), logo_line);
        }

        scalony_string.push_str(&prefix);
        scalony_string.push_str(result_line);

        final_vector.push(scalony_string);
    }

    final_vector
}

fn print_colored(text: &str, colors: Vec<Color>) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut color_spec = ColorSpec::new();
    
    // Ustawienie domyślnego koloru (pierwszy element wektora)
    if let Some(default_color) = colors.get(0) {
        color_spec.set_fg(Some(*default_color));
        stdout.set_color(&color_spec)?;
    }
    
    let mut buffer = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            // Sprawdzenie, czy następny znak to cyfra
            if let Some(&next_char) = chars.peek() {
                if next_char.is_ascii_digit() {
                    chars.next();
                    let index = next_char.to_digit(10).unwrap() as usize - 1;
                    if let Some(color) = colors.get(index) {
                        // Wydrukowanie zbuforowanego tekstu z poprzednim kolorem
                        write!(stdout, "{}", buffer)?;
                        buffer.clear();
                        // Ustawienie nowego koloru
                        stdout.flush()?;
                        color_spec.set_fg(Some(*color));
                        stdout.set_color(&color_spec)?;
                    }
                    continue;
                }
            }
        }
        buffer.push(c);
    }
    
    // Wydrukowanie pozostałego tekstu
    write!(stdout, "{}", buffer)?;
    stdout.reset()?;
    write!(stdout, "\n")?;
    stdout.flush()?;
    Ok(())
}

fn read_logo(logo: Option<String>) -> String {
    if let Some(logo_value) = logo.as_deref(){
        let home_dir = env::var("HOME").expect("Unable to find home directory");
        let path = format!("{}/.config/rastfetch/{}", home_dir, logo_value);

        match fs::read_to_string(path){
            Ok(contents) => contents.to_string(),
            Err(e) => "Error reading logo file: ".to_string() + &e.to_string(),
        }
    }else{
        let os = System::name().unwrap_or("Can't find system name".to_string());
        let os_short =  *os_map::OS_MAP.get(&os).unwrap_or(&"unknown");
        let path = format!("logo/ascii/{}.txt", os_short);
        
        if let Some(file) = ASSETS.get_file(path) {
            let contents = file.contents_utf8().unwrap();
            contents.to_string()
        } else {
            "Default logo not found".to_string()
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