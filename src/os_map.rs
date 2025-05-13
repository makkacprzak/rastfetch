use phf::phf_map;
use termcolor::Color;

pub static OS_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "Fedora Linux" => "fedora",
    "Ubuntu Linux" => "ubuntu"
};

pub static OS_COLORS: phf::Map<&'static str, &'static [Color]> = phf_map! {
    "Fedora Linux" => &[Color::Blue, Color::White],
    "Ubuntu Linux" => &[Color::Red, Color::White]
};