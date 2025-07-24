#!/bin/bash

# rFetch Installation Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rust is installed
check_rust() {
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo is not installed!"
        print_status "Please install Rust from https://rustup.rs/"
        exit 1
    fi
    print_success "Rust/Cargo found"
}

# Build the project
build_project() {
    print_status "Building rFetch in release mode..."
    cargo build --release
    print_success "Build completed"
}

# Install the binary
install_binary() {
    local install_dir="/usr/local/bin"
    local binary_name="rfetch"
    
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        install_dir="/usr/local/bin"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        install_dir="/usr/local/bin"
    else
        print_warning "Unknown OS type, using default install directory"
    fi

    print_status "Installing rFetch to $install_dir..."
    
    # Check if we need sudo
    if [[ -w "$install_dir" ]]; then
        cp "target/release/$binary_name" "$install_dir/"
    else
        print_status "Requesting sudo privileges for installation..."
        sudo cp "target/release/$binary_name" "$install_dir/"
    fi
    
    # Make sure it's executable
    if [[ -w "$install_dir/$binary_name" ]]; then
        chmod +x "$install_dir/$binary_name"
    else
        sudo chmod +x "$install_dir/$binary_name"
    fi
    
    print_success "rFetch installed to $install_dir/$binary_name"
}

# Create config directory and copy example config
setup_config() {
    local config_dir
    
    if [[ "$OSTYPE" == "darwin"* ]] || [[ "$OSTYPE" == "linux-gnu"* ]]; then
        config_dir="$HOME/.config/rfetch"
    else
        config_dir="$HOME/.rfetch"
    fi
    
    print_status "Setting up configuration directory at $config_dir..."
    
    mkdir -p "$config_dir"
    
    if [[ -f "config.example.toml" ]] && [[ ! -f "$config_dir/config.toml" ]]; then
        cp "config.example.toml" "$config_dir/config.toml"
        print_success "Example configuration copied to $config_dir/config.toml"
    else
        print_warning "Configuration file already exists or example not found"
    fi
}

# Test installation
test_installation() {
    print_status "Testing installation..."
    
    if command -v rfetch &> /dev/null; then
        print_success "rFetch is installed and available in PATH"
        print_status "Running version check..."
        rfetch --version
    else
        print_error "rFetch installation failed or not in PATH"
        exit 1
    fi
}

# Main installation process
main() {
    echo "=================================="
    echo "    rFetch Installation Script    "
    echo "=================================="
    echo
    
    check_rust
    build_project
    install_binary
    setup_config
    test_installation
    
    echo
    print_success "Installation completed successfully!"
    echo
    print_status "You can now run 'rfetch' from anywhere in your terminal"
    print_status "Configuration file is located at ~/.config/rfetch/config.toml"
    print_status "Run 'rfetch --help' for usage information"
    echo
}

# Run main function
main "$@"