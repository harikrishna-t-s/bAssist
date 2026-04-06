#!/bin/bash

# bAssist Installation Script
# This script installs, updates, or uninstalls bAssist terminal command assistant tool

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BASSIST_VERSION="0.1.0"
INSTALL_DIR="/usr/local/bin"
BACKUP_DIR="$HOME/.bassist/backups"
CONFIG_DIR="$HOME/.bassist"
REPO_URL="https://github.com/user/bassist.git"
RELEASE_API="https://api.github.com/repos/user/bassist/releases"

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

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE} bAssist - Terminal Command Assistant${NC}"
    echo -e "${BLUE}================================${NC}"
}

# Exception handling
handle_error() {
    local exit_code=$?
    local line_number=$1
    print_error "Script failed at line $line_number with exit code $exit_code"
    cleanup_on_error
    exit $exit_code
}

cleanup_on_error() {
    print_info "Cleaning up temporary files..."
    if [ -n "$TEMP_DIR" ] && [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
    fi
}

# Set up error handling
trap 'handle_error $LINENO' ERR

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

# Create backup of current installation
create_backup() {
    if [ -f "$INSTALL_DIR/bassist" ]; then
        print_info "Creating backup of current installation..."
        mkdir -p "$BACKUP_DIR"
        local backup_name="bassist_backup_$(date +%Y%m%d_%H%M%S)"
        cp "$INSTALL_DIR/bassist" "$BACKUP_DIR/$backup_name"
        echo "$backup_name" > "$BACKUP_DIR/last_backup"
        print_info "Backup created: $BACKUP_DIR/$backup_name"
    fi
}

# Restore from backup
restore_backup() {
    if [ -f "$BACKUP_DIR/last_backup" ]; then
        local last_backup=$(cat "$BACKUP_DIR/last_backup")
        if [ -f "$BACKUP_DIR/$last_backup" ]; then
            print_info "Restoring from backup: $last_backup"
            cp "$BACKUP_DIR/$last_backup" "$INSTALL_DIR/bassist"
            print_info "Restore completed successfully"
            return 0
        fi
    fi
    print_error "No backup found"
    return 1
}

# Check for updates
check_updates() {
    print_info "Checking for updates..."
    
    if ! command -v curl &> /dev/null; then
        print_warning "curl not found, cannot check for updates"
        return 1
    fi
    
    # Get latest version from GitHub API
    local latest_version
    latest_version=$(curl -s "$RELEASE_API/latest" | grep '"tag_name"' | cut -d'"' -f4 2>/dev/null || echo "")
    
    if [ -z "$latest_version" ]; then
        print_warning "Could not check for updates (network issue?)"
        return 1
    fi
    
    local current_version="unknown"
    if [ -f "$INSTALL_DIR/bassist" ]; then
        current_version=$("$INSTALL_DIR/bassist" --version 2>/dev/null | cut -d' ' -f2 || echo "unknown")
    fi
    
    print_info "Current version: $current_version"
    print_info "Latest version: $latest_version"
    
    if [ "$current_version" != "$latest_version" ]; then
        print_info "Update available: $latest_version"
        return 0
    else
        print_info "bAssist is up to date"
        return 1
    fi
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
        # Check if we're already in the bassist directory
        if [ -f "../Cargo.toml" ] && grep -q "bassist" "../Cargo.toml"; then
            print_info "Using local source code..."
            cp -r ../* .
        else
            git clone "$REPO_URL" .
        fi
    else
        print_error "Git is not installed. Please install Git first."
        exit 1
    fi
    
    # Build the project
    print_info "Building bAssist..."
    cargo build --release
    
    # Install to system
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

# Uninstall bAssist
uninstall() {
    print_info "Uninstalling bAssist..."
    
    # Remove binary
    if [ -f "$INSTALL_DIR/bassist" ]; then
        rm "$INSTALL_DIR/bassist"
        print_info "Removed binary from $INSTALL_DIR/bassist"
    fi
    
    if [ -f "$HOME/.local/bin/bassist" ]; then
        rm "$HOME/.local/bin/bassist"
        print_info "Removed binary from ~/.local/bin/bassist"
    fi
    
    # Ask about config directory
    if [ -d "$CONFIG_DIR" ]; then
        echo -n "Remove configuration directory $CONFIG_DIR? (y/N): "
        read -r response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            rm -rf "$CONFIG_DIR"
            print_info "Removed configuration directory"
        else
            print_info "Configuration directory preserved"
        fi
    fi
    
    # Ask about backups
    if [ -d "$BACKUP_DIR" ]; then
        echo -n "Remove backup directory $BACKUP_DIR? (y/N): "
        read -r response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            rm -rf "$BACKUP_DIR"
            print_info "Removed backup directory"
        else
            print_info "Backup directory preserved"
        fi
    fi
    
    print_info "bAssist uninstalled successfully"
}

# Rollback to previous version
rollback() {
    print_info "Rolling back to previous version..."
    
    if restore_backup; then
        print_info "Rollback completed successfully"
    else
        print_error "Rollback failed"
        exit 1
    fi
}

# Update bAssist
update() {
    print_info "Updating bAssist..."
    
    # Check for updates first
    if ! check_updates; then
        print_info "No updates available"
        return 0
    fi
    
    # Create backup before update
    create_backup
    
    # Perform update
    install_from_source
    
    print_info "Update completed successfully"
}

# Verify installation
verify_installation() {
    if command -v bassist &> /dev/null; then
        local version
        version=$(bassist --version 2>/dev/null || echo "unknown")
        print_info "bAssist is now installed! Version: $version"
        echo ""
        echo "Usage:"
        echo "  bassist                    # Launch TUI interface"
        echo "  bassist -tui              # Launch TUI interface"
        echo "  bassist -s 'git add'      # Search for git add commands"
        echo "  bassist -x 'git status'   # Execute command directly"
        echo "  bassist -e 'git add'      # Explain command"
        echo "  bassist --dry-run -x 'rm -rf' # Dry run execution"
        echo "  bassist --best-practice 'terraform destroy' # Show best practices"
        echo "  bassist alias add 'll' 'ls -la'  # Add alias"
        echo "  bassist alias list         # List all aliases"
        echo ""
        print_info "Run 'bassist --help' for more information."
        return 0
    else
        print_error "Installation failed. bAssist is not in PATH."
        return 1
    fi
}

# Show help
show_help() {
    print_header
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  install      Install bAssist (default)"
    echo "  update       Update to latest version"
    echo "  uninstall    Uninstall bAssist"
    echo "  rollback     Rollback to previous version"
    echo "  backup       Create backup of current installation"
    echo "  restore      Restore from backup"
    echo "  check        Check for updates"
    echo "  help         Show this help message"
    echo ""
    echo "Options:"
    echo "  --force      Force installation/upgrade"
    echo "  --backup     Create backup before installing"
    echo "  --dev        Install from source (development mode)"
    echo ""
    echo "Examples:"
    echo "  $0 install              # Install bAssist"
    echo "  $0 update               # Update to latest version"
    echo "  $0 uninstall            # Uninstall bAssist"
    echo "  $0 rollback             # Rollback to previous version"
    echo "  $0 install --backup     # Install with backup"
    echo ""
}

# Main installation process
main() {
    local command=${1:-install}
    local force_mode=false
    local backup_mode=false
    local dev_mode=false
    
    # Parse arguments
    shift
    while [[ $# -gt 0 ]]; do
        case $1 in
            --force)
                force_mode=true
                shift
                ;;
            --backup)
                backup_mode=true
                shift
                ;;
            --dev)
                dev_mode=true
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    print_header
    
    case $command in
        install)
            print_info "Starting bAssist installation..."
            
            # Check prerequisites
            check_rust
            
            # Get system info
            local arch=$(get_arch)
            local os=$(get_os)
            print_info "Detected system: $os ($arch)"
            
            # Check if already installed
            if [ -f "$INSTALL_DIR/bassist" ] || [ -f "$HOME/.local/bin/bassist" ]; then
                if [ "$force_mode" = true ]; then
                    print_warning "bAssist already installed, forcing reinstall..."
                else
                    print_warning "bAssist is already installed. Use 'update' to upgrade or '--force' to reinstall."
                    exit 1
                fi
            fi
            
            # Create backup if requested
            if [ "$backup_mode" = true ]; then
                create_backup
            fi
            
            # Install
            install_from_source
            
            # Setup configuration
            setup_config
            
            # Verify installation
            verify_installation
            ;;
        update)
            print_info "Starting bAssist update..."
            update
            verify_installation
            ;;
        uninstall)
            uninstall
            ;;
        rollback)
            rollback
            ;;
        backup)
            create_backup
            ;;
        restore)
            restore_backup
            ;;
        check)
            check_updates
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
    
    print_info "Operation completed successfully!"
}

# Setup configuration directory
setup_config() {
    if [ ! -d "$CONFIG_DIR" ]; then
        mkdir -p "$CONFIG_DIR"
        print_info "Created configuration directory: $CONFIG_DIR"
    fi
}

# Run main function
main "$@"
