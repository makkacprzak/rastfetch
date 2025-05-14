# Description

Rastfetch is a clone of [fastfetch](https://github.com/fastfetch-cli/fastfetch) written in Rust. It was started as an excercise to learn the basics of Rust, but it appears it could potentially be faster tthan fastfetch.

# Install

1) Clone github repo
```bash
git clone https://github.com/makkacprzak/rastfetch.git && cd rastfetch
```
2) Install Rust and Cargo with your package manager
3) Build using
```bash
cargo buld --release && cd target/release
```
4) (Optional) Make a symlink to folder on PATH, for example
```bash
ln -s ./rastfetch /usr/local/bin/rastfetch
```
5) Run rastfetch
```bash
# If you created a symlink
rasftetch
# If you didn't make a symlink
cd [path/to/rastfetch/bin]
./rastfetch
```

# How to use

1) Rastfetch looks for a configuration file under  `~/.config/rastfetch/config.json`.
2) It sould automatically create a file on this path, but if it doesn't you can create it with either
```bash
# either
rastfetch -c
# or
mkdir ~/.config/rastfetch/
cd ~/.config/rastfetch/
# with vim
vim config.json
# with nano
nano config.json
```
3) Copy default config from [default.json](doc/default.json)
4) You can find all available modules in [modules.md](doc/modules.md). To change which modules are used just and in which order, just add them to the config.json

# Change logo displayed
#### Rastfetch currently supports:
1) Colored ascii art courtesy of [fastfetch](https://github.com/fastfetch-cli/fastfetch). You can see all available logos in [ascii](assets/logo/ascii). To run with one of these stock logos run:
```bash
rastfetch -l [logo_name_without_.txt]
```
2) Personal custom logos in:
* ansi format
* normal ascii
* colored ascii with `$[1-9]` symbols. To see examples check out [ascii](assets/logo/ascii), to check colors see [colors.md](doc/colors.md)

To use a custom logo, first place it in your `~/.config/rastfetch` directory, and then run:
```bash
rastfetch -lc [file_name]
```

# Contributions

Contributions meant to expand the list of available modules, increase speed or efficiency or increase support for different distros are very much welcome. All contributions will automaticlly be licensed under the project's [MIT](LICENSE.md) license.

# Disclosure

As I am not 'REALLY' an experienced programmer, this project uses many external libraries. The long term goal is, once this project comes to a mostly finished state, to slowly phase out external libraries and create new ones.

# License

#### 1. This project is licensed under the [MIT](LICENSE.md) license
2) This project is inspired by [fastfetch](https://github.com/fastfetch-cli/fastfetch). It also uses fastfetch's lbrary of ansi logos. It is licensed under the [MIT](LICENSE.md) license
3) This project uses [clap](https://github.com/clap-rs/clap) for handling of CLI arguments. It is licensed under the [MIT](LICENSE.md) license
4) This project uses [serde](https://github.com/serde-rs/serde) and [serde_json](https://github.com/serde-rs/json) for json parsing. It is licensed under the [MIT](LICENSE.md) license
5) This project uses [sysinfo](https://github.com/GuillaumeGomez/sysinfo) for fetching system information. It is licensed under the [MIT](LICENSE.md)
6) This project uses [phf](https://github.com/rust-phf/rust-phf) for generating efficient lookup tables at compile time. It is licensed under the [MIT](LICENSE.md) license
7) This project uses [include_dir](https://github.com/Michael-F-Bryan/include_dir) for including ascii logos in the binary for speed. It is licensed under the [MIT](LICENSE.md) license
8) This project uses [tokio](https://github.com/tokio-rs/tokio) for handling multithreading. It is licensed under the [MIT](LICENSE.md) license
9) This project uses [whoami](https://github.com/ardaku/whoami) for fetching specific user information faster. It is licensed under the [MIT](LICENSE.md) license
10) This project uses [strip-ansi-escapes](https://github.com/luser/strip-ansi-escapes) for processing ansi images. It is licensed under the [MIT](LICENSE.md) license
11) This project uses [termcolor]{https://github.com/BurntSushi/termcolor} for writing colored text to terminal. It is licensed under the [MIT]{LICENSE.md} license
