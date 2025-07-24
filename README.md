# rFetch

A fast and beautiful system information tool written in Rust - an improved version of fastfetch.

## Features

- 🚀 **Fast**: Written in Rust for optimal performance
- 🎨 **Beautiful**: Colorful ASCII logos for many distributions
- 🔧 **Configurable**: Flexible TOML configuration file
- 🌍 **Cross-platform**: Supports Linux, macOS, Windows and iOS
- 📊 **Complete information**: CPU, GPU, memory, disk, battery and more
- 🎯 **Multiple modes**: Normal, minimal, verbose and JSON
- 🌈 **Customizable themes**: Built-in theme system with colors and layouts
- 💾 **Enhanced GPU and SSD**: Advanced GPU detection and detailed disk information
- 📱 **iOS Support**: Full iOS support with automatic detection and iSH compatibility

## Installation

### Pre-built binaries

Download the latest pre-built binaries from the [Releases](https://github.com/dddevid/rFetch/releases) page:

- **macOS/Linux**: `rfetch-macos-linux`
- **Windows**: `rfetch-windows.exe`

```bash
# macOS/Linux
chmod +x rfetch-macos-linux
./rfetch-macos-linux

# Windows
rfetch-windows.exe
```

### From source

```bash
git clone https://github.com/dddevid/rFetch.git
cd rFetch
cargo build --release
sudo cp target/release/rfetch /usr/local/bin/
```

### Cross-compilation for Windows

To build a Windows executable from macOS/Linux:

```bash
# Install the Windows target
rustup target add x86_64-pc-windows-gnu

# Install MinGW linker (macOS with Homebrew)
brew install mingw-w64

# Create Cargo configuration for Windows linking
mkdir -p .cargo
echo '[target.x86_64-pc-windows-gnu]' > .cargo/config.toml
echo 'linker = "x86_64-w64-mingw32-gcc"' >> .cargo/config.toml

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu

# The Windows executable will be at:
# target/x86_64-pc-windows-gnu/release/rfetch.exe
```

### With Cargo

```bash
cargo install rfetch
```

### On iOS (iSH)

```bash
# Install Rust and necessary dependencies
apk update && apk upgrade
apk add rust cargo git

# Clone and build
git clone https://github.com/dddevid/rFetch.git
cd rFetch
cargo build --release

# Copy the executable
cp target/release/rfetch /usr/local/bin/
```

## Running on Different Platforms

### Arch Linux (and based distros)

```bash
# Install from AUR (if available)
yay -S rfetch

# Or build from source
sudo pacman -S rust git
git clone https://github.com/dddevid/rFetch.git
cd rFetch
cargo build --release
sudo cp target/release/rfetch /usr/local/bin/

# Run
rfetch
```

### Ubuntu (and based distros)

```bash
# Install dependencies
sudo apt update
sudo apt install curl build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build from source
git clone https://github.com/dddevid/rFetch.git
cd rFetch
cargo build --release
sudo cp target/release/rfetch /usr/local/bin/

# Run
rfetch
```

### macOS

```bash
# Install Rust via Homebrew (recommended)
brew install rust

# Or install Rust directly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build from source
git clone https://github.com/dddevid/rFetch.git
cd rFetch
cargo build --release
sudo cp target/release/rfetch /usr/local/bin/

# Run
rfetch
```

### Windows

```bash
# Download the pre-built executable from releases
# Or build from source with Rust installed

# Using PowerShell/Command Prompt
.\rfetch.exe

# Or add to PATH and run
rfetch
```

### Termux (Android)

```bash
# Update packages
pkg update && pkg upgrade

# Install dependencies
pkg install rust git

# Clone and build
git clone https://github.com/dddevid/rFetch.git
cd rFetch
cargo build --release

# Copy to bin directory
cp target/release/rfetch $PREFIX/bin/

# Run
rfetch
```

## Usage

### Basic usage

```bash
rfetch
```

### How to run rFetch

#### After downloading pre-built binaries

```bash
# macOS/Linux - Make executable and run
chmod +x rfetch-macos-linux
./rfetch-macos-linux

# Windows - Run directly
rfetch-windows.exe

# Or rename and add to PATH
mv rfetch-windows.exe rfetch.exe
# Add directory to Windows PATH, then:
rfetch
```

#### After building from source

```bash
# Run directly from target directory
./target/release/rfetch

# Or install system-wide
sudo cp target/release/rfetch /usr/local/bin/
rfetch

# Windows (after cross-compilation)
./target/x86_64-pc-windows-gnu/release/rfetch.exe
```

#### Platform-specific execution

```bash
# Linux/macOS - Standard execution
rfetch

# Termux (Android)
rfetch

# Windows Command Prompt
rfetch.exe

# Windows PowerShell
.\rfetch.exe
```

### Available options

```bash
rfetch --help
```

- `-c, --config <FILE>`: Use a custom configuration file
- `-l, --logo <LOGO>`: Logo type (auto, ascii, small, none)
- `--color <WHEN>`: When to use colors (auto, always, never)
- `-j, --json`: Output in JSON format
- `-m, --minimal`: Show minimal information
- `-v, --verbose`: Show verbose information
- `--theme <THEME>`: Use a specific theme
- `--list-themes`: List all available themes

### Examples

```bash
# Minimal output
rfetch --minimal

# JSON output
rfetch --json

# Specific logo
rfetch --logo small

# No colors
rfetch --color never

# Use neon theme
rfetch --theme neon

# List all themes
rfetch --list-themes

# Custom configuration
rfetch --config ~/.config/rfetch/custom.toml
```

## Themes

rFetch includes a built-in theme system with several predefined themes:

- **default**: Default theme with balanced colors
- **minimal**: Clean and minimal theme with essential information
- **neon**: Bright and vivid colors for a modern look
- **retro**: Vintage style with green colors

### Using themes

```bash
# List all available themes
rfetch --list-themes

# Use a specific theme
rfetch --theme neon
rfetch --theme minimal
rfetch --theme retro
```

## Configuration

rFetch uses a TOML configuration file. The default file is located at:

- Linux/macOS: `~/.config/rfetch/config.toml`
- Windows: `%APPDATA%\rfetch\config.toml`

### Configuration example

```toml
[display]
logo_type = "auto"
color_mode = "auto"
output_format = "normal"
minimal = false
verbose = false
separator = ": "
padding = 2

[info]
show_os = true
show_kernel = true
show_uptime = true
show_packages = true
show_shell = true
show_resolution = true
show_de = true
show_wm = true
show_theme = false
show_icons = false
show_font = false
show_cursor = false
show_terminal = true
show_cpu = true
show_gpu = true
show_memory = true
show_disk = true
show_battery = true
show_locale = false
show_local_ip = false
show_public_ip = false
show_users = false
show_date = true

[colors]
title = "cyan"
subtitle = "blue"
key = "yellow"
value = "white"
separator = "white"
logo = "cyan"
```

## Supported information

- **System**: OS, kernel, uptime
- **Hardware**: CPU, GPU (with core count), memory, disk (with used/total space), battery
- **Software**: Shell, terminal, DE, WM, packages
- **Network**: Local IP, public IP
- **Other**: Resolution, theme, font, users, date

### GPU improvements

- **macOS**: Apple Silicon detection (M1, M2, etc.) with core count
- **Linux**: Enhanced support for discrete and integrated GPUs
- **Windows**: GPU detection via WMI

### Disk improvements

- **Enhanced format**: Shows "Used / Total (Percentage%)"
- **Accurate parsing**: Correct handling of units (GB, TB, etc.)
- **Multi-platform**: Optimized support for Linux, macOS and Windows

## Supported operating systems

### Linux
- Arch Linux
- Ubuntu
- Fedora
- Debian
- Gentoo
- Manjaro
- openSUSE
- CentOS
- Alpine
- And many others

### iOS
- Complete iOS support with automatic detection
- Automatic Termux environment detection
- Package management with `pkg`
- iOS-specific CPU and GPU information
- Dedicated iOS logo
- Full iSH shell compatibility

### macOS
- Complete macOS/Darwin support
- Optimized Apple Silicon detection
- Accurate disk information

### Windows
- Basic Windows support
- GPU detection via WMI
- Disk information via WMI

## Development

### Prerequisites

- Rust 1.70+
- Cargo

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Release

```bash
cargo build --release
```

## Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Theme Creator

Create custom themes with our interactive Theme Creator:
**[rFetch Theme Creator](https://dddevid.github.io/rFetch-theme-creator/)**

Features:
- Real-time terminal preview
- Dark/light mode with auto-detection
- Live color and effect updates
- Export to YAML/TOML formats
- Modern responsive UI

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgments

- Inspired by [fastfetch](https://github.com/fastfetch-cli/fastfetch)
- ASCII logos from various community sources
- Rust community for excellent libraries

## Roadmap

- [ ] Support for more package managers
- [ ] User-defined custom themes
- [ ] Plugin system for additional information
- [ ] GUI configuration
- [ ] Support for more output formats
- [ ] Plugin system
- [ ] Predefined themes
- [ ] Support for images in terminals
- [ ] Benchmarks and optimizations
- [ ] Container support
- [ ] API for integration with other tools