//! This module contains all perfect hash maps used for localising rastfetch for different distros and systems
use phf::phf_map;
use termcolor::Color;

/// A static perfect hash map, maping the distro's ID found in /etc/os-release (or in case of MacOs its kernel's name) to its logo's name
pub static OS_LOGO: phf::Map<&'static str, &'static str> = phf_map! {
    "fedora" => "fedora",
    "ubuntu" => "ubuntu",
    "bazzite" => "bazzite_full",
    "Darwin" => "macos",
    "zorin" => "zorin",
    "arch" => "arch",
    "debian" => "debian",
    "linuxmint" => "linuxmint",
    "endeavouros" => "endeavouros"
};

/// A static perfect hash map, maping the distro's ID found in /etc/os-release (or in case of MacOs its kernel's name) to its color palette
pub static OS_COLORS: phf::Map<&'static str, &'static [Color]> = phf_map! {
    "fedora" => &[Color::Ansi256(12), Color::White, Color::Blue, Color::Green],
    "ubuntu" => &[Color::Rgb(255, 69, 0), Color::White, Color::Red, Color::Green],
    "bazzite" => &[Color::Rgb(8, 83, 174), Color::White, Color::Rgb(186, 43, 226), Color::Green],
    "Macos" => &[Color::Green,Color::White, Color::Yellow, Color::Red, Color::Magenta, Color::Cyan],
    "zorin" => &[Color::Cyan, Color::White, Color::Cyan, Color::Green],
    "arch" => &[Color::Blue, Color::White, Color::Blue, Color::Green],
    "debian" => &[Color::Red, Color::White, Color::Red, Color::Green],
    "linuxmint" => &[Color::Ansi256(10), Color::White, Color::Green, Color::Green],
    "endeavouros" => &[Color::Ansi256(9), Color::White, Color::Magenta, Color::Ansi256(12)],
};

pub static CHASSIS_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Other",
    "2" => "Unknown",
    "3" => "Desktop",
    "4" => "Low Profile Desktop",
    "5" => "Pizza Box",
    "6" => "Mini Tower",
    "7" => "Tower",
    "8" => "Portable",
    "9" => "Laptop",
    "10" => "Notebook",
    "11" => "Handheld",
    "12" => "Docking Station",
    "13" => "All in One",
    "14" => "Sub Notebook",
    "15" => "Space-saving",
    "16" => "Lunch Box",
    "17" => "Main System Chassis",
    "18" => "Expansion Chassis",
    "19" => "SubChassis",
    "20" => "Bus Expansion Chassis",
    "21" => "Periferal Chassis",
    "22" => "Storage Chassis",
    "23" => "Rack Mount Chassis",
    "24" => "Sealed-Case PC",
    "25" => "Multi-system Chassis",
    "26" => "Compact PCI",
    "27" => "Advanced TCA",
    "28" => "Blade",
    "29" => "Blade Enclosure",
    "30" => "Tablet",
    "31" => "Convertible",
    "32" => "Detachable",
    "33" => "IoT Gateway",
    "34" => "Embedded PC",
    "35" => "Mini PC",
    "36" => "Stick PC",
    "Unknown" => "Unknown"
};

/* this part is temporarily disabled
the function using it is too slow
pub static SHELL_VERSIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "bash" => "echo $BASH_VERSION",
    "zsh" => "echo $ZSH_VERSION",
    "fish" => "echo $FISH_VERSION",
    "zh" => "echo $ZSH_VERSION",
    "sh" => "echo $BASH_VERSION"
};
*/