# For contributors

Colors and logos associated with different distros are stored in [src/os_map.rs](/src/os_map.rs). 

***IMPORTANT!*** *Distro names* (with the exception of macos) are specifically found by running `cat /etc/os-release`, and seeing what appears under the `NAME=` tag.

# OS_LOGO

Here you can map *distro name* to its logo located in [assets/logo/ascii](/assets/logo/ascii). Remember to omit the **.txt** file extension.

# OS_COLORS

Here you can map *distro name* to its color pallete. Here are some things to remember: 
* If no $[num] marker is given, the default is always **$1**.
* $1 tag is used for **title** accent color, $2 is typically white because this is used for normal text, $3 is used for all other module headers  like "Kernel:" and "OS:". This convention is roughly what's used by fastfetch, so you should be able to reproduce fastfetch's style for every distro 1:1
* When designing a pallete you should strive to reproduce fastfetch's style
* All pallete tables must contain between 3 and 9 colors.
* Some images in [assets/logo/ascii](/assets/logo/ascii) need to be adapted before working properly. In fastfetch selected colors carry over line by line, whereas here if no $[num] tag is given at the start of a line, it will always default to $1. For example compare [macos-fastfetch](/assets/logo/ascii/macos-fasfetch.txt) with [macos-rastfetch](/assets/logo/ascii/macos.txt). 

# Colors

Color table positions are interpreted as positive integers, so ex. Fedora[0] color will be drawn when marker $1 appears, etc.

1) Fedora - [Blue, White, Blue]
2) Ubuntu - [Red, White, Red]
3) Bazzite - [Magenta, White, Magenta]
4) Darwin - [Green, White, Yellow, Red, Magenta, Cyan]
