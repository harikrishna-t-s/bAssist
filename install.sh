#!/bin/bash

# bAssist Installation Script
# This script installs bAssist terminal command assistant tool

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print functions
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
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
        print_error "Rust is not installed. Please install Rust first:"
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    print_info "Rust is installed"
}

# Check system architecture
get_arch() {
    case "$(uname -m)" in
        x86_64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "aarch64"
            ;;
        *)
            print_error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
}

# Get operating system
get_os() {
    case "$(uname -s)" in
        Linux*)
            echo "linux"
            ;;
        Darwin*)
            echo "macos"
            ;;
        *)
            print_error "Unsupported operating system: $(uname -s)"
            exit 1
            ;;
    esac
}

# Install from source
install_from_source() {
    print_info "Installing bAssist from source..."
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    # Clone or download source
    if command -v git &> /dev/null; then
        print_info "Cloning from repository..."
        git clone https://github.com/user/bassist.git .
    else
        print_error "Git is not installed. Please install Git first."
        exit 1
    fi
    
    # Build the project
    print_info "Building bAssist..."
    cargo build --release
    
    # Install to system
    INSTALL_DIR="/usr/local/bin"
    if [ -w "$INSTALL_DIR" ]; then
        cp target/release/bassist "$INSTALL_DIR/"
        print_info "bAssist installed to $INSTALL_DIR/bassist"
    else
        print_warning "Cannot write to $INSTALL_DIR. Installing to ~/.local/bin..."
        mkdir -p "$HOME/.local/bin"
        cp target/release/bassist "$HOME/.local/bin/"
        
        # Add to PATH if not already there
        if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
            print_warning "Added ~/.local/bin to PATH. Please restart your shell or run: source ~/.bashrc"
        fi
        print_info "bAssist installed to ~/.local/bin/bassist"
    fi
    
    # Cleanup
    cd /
    rm -rf "$TEMP_DIR"
}

# Create configuration directory
setup_config() {
    CONFIG_DIR="$HOME/.bassist"
    if [ ! -d "$CONFIG_DIR" ]; then
        mkdir -p "$CONFIG_DIR"
        print_info "Created configuration directory: $CONFIG_DIR"
    fi
}

# Verify installation
verify_installation() {
    if command -v bassist &> /dev/null; then
        print_info "bAssist is now installed!"
        echo ""
        echo "Usage:"
        echo "  bassist                    # Launch TUI interface"
        echo "  bassist -tui              # Launch TUI interface"
        echo "  bassist -s 'git add'      # Search for git add commands"
        echo "  bassist alias add 'll' 'ls -la'  # Add alias"
        echo "  bassist alias list         # List all aliases"
        echo ""
        print_info "Run 'bassist --help' for more information."
    else
        print_error "Installation failed. bAssist is not in PATH."
        exit 1
    fi
}

# Main installation process
main() {
    print_info "Starting bAssist installation..."
    
    # Check prerequisites
    check_rust
    
    # Get system info
    ARCH=$(get_arch)
    OS=$(get_os)
    print_info "Detected system: $OS ($ARCH)"
    
    # Install
    install_from_source
    
    # Setup configuration
    setup_config
    
    # Verify installation
    verify_installation
    
    print_info "Installation completed successfully!"
}

# Run main function
main "$@"
