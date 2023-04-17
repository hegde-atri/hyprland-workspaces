# Hyprland-workspaces

[![Licence](https://img.shields.io/github/license/hegde-atri/hyprland-workspaces?color=red)](https://github.com/hegde-atri/hyprland-workspaces/blob/main/LICENCE)
[![Version](https://img.shields.io/crates/v/hyprland-workspaces?color=9cf)](https://crates.io/crates/hyprland-workspaces/versions)
[![Downloads](https://img.shields.io/crates/d/hyprland-workspaces)](https://crates.io/crates/hyprland-workspaces)



An application written in rust that uses `hyprctl` to return workspace data.

# Preface 

I needed workspace data to display them on my `eww` bar.

I made this as I couldn't write shell script to do this, and I was having issues with the default script from [[https://wiki.hyprland.org][Hyprland's website]].

# Prerequisites

- Hyprland (Program uses `hyprctl`)

# How to install

You can install this by `cargo` or building the project yourself

``` sh
# Using cargo
cargo install hyprland-workspaces
```

```sh
# Cloning and building
git clone https://github.com/hegde-atri/hyprland-workspaces
cd hyprland-workspaces
cargo build --release
```
