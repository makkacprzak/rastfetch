use phf::phf_map;
use termcolor::Color;

pub static OS_LOGO: phf::Map<&'static str, &'static str> = phf_map! {
    "Fedora Linux" => "fedora",
    "Ubuntu Linux" => "ubuntu"
};

pub static OS_COLORS: phf::Map<&'static str, &'static [Color]> = phf_map! {
    "Fedora Linux" => &[Color::Blue, Color::White],
    "Ubuntu Linux" => &[Color::Red, Color::White],
    "Bazzite" => &[Color::Green, Color::White],
};

pub static SHELL_VERSIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "bash" => "echo $BASH_VERSION",
    "zsh" => "echo $ZSH_VERSION",
    "fish" => "echo $FISH_VERSION",
    "zh" => "echo $ZSH_VERSION",
    "sh" => "echo $BASH_VERSION"
};