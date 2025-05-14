use phf::phf_map;
use termcolor::Color;

pub static OS_LOGO: phf::Map<&'static str, &'static str> = phf_map! {
    "Fedora Linux" => "fedora",
    "Ubuntu Linux" => "ubuntu",
    "Bazzite" => "bazzite_full",
    "Darwin" => "macos"
};

pub static OS_COLORS: phf::Map<&'static str, &'static [Color]> = phf_map! {
    "Fedora Linux" => &[Color::Blue, Color::White, Color::Blue, Color::Green],
    "Ubuntu Linux" => &[Color::Red, Color::White, Color::Red, Color::Green],
    "Bazzite" => &[Color::Ansi256(5), Color::White, Color::Ansi256(13), Color::Green],
    "Darwin" => &[Color::Green,Color::White, Color::Yellow, Color::Red, Color::Magenta, Color::Cyan]
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