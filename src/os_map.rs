use phf::phf_map;
use termcolor::Color;

pub static OS_LOGO: phf::Map<&'static str, &'static str> = phf_map! {
    "Fedora Linux" => "fedora",
    "Ubuntu Linux" => "ubuntu",
    "Darwin" => "macos"
};

pub static OS_COLORS: phf::Map<&'static str, &'static [Color]> = phf_map! {
    "Fedora Linux" => &[Color::Blue, Color::White, Color::Blue],
    "Ubuntu Linux" => &[Color::Red, Color::White, Color::Red],
    "Bazzite" => &[Color::Magenta, Color::White, Color::Magenta],
    "Darwin" => &[Color::Green,Color::White, Color::Yellow, Color::Red, Color::Magenta, Color::Cyan]
};

pub static SHELL_VERSIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "bash" => "echo $BASH_VERSION",
    "zsh" => "echo $ZSH_VERSION",
    "fish" => "echo $FISH_VERSION",
    "zh" => "echo $ZSH_VERSION",
    "sh" => "echo $BASH_VERSION"
};