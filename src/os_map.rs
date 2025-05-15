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