# bAssist Installation Guide

## Quick Start

### One-Command Installation
```bash
curl -sSL https://raw.githubusercontent.com/user/bassist/main/install.sh | bash
```

### Download and Install
```bash
# Download installation script
curl -O https://raw.githubusercontent.com/user/bassist/main/install.sh
chmod +x install.sh

# Install bAssist
./install.sh install
```

## System Requirements

### Required
- **Rust 1.70+** - For building from source
- **Git** - For cloning repository
- **curl** - For checking updates (optional)
- **Bash** - For installation script

### Supported Systems
- **Linux** (x86_64, aarch64)
- **macOS** (x86_64, aarch64)
- **Windows** (via WSL)

### Optional Dependencies
- **Docker** - For container commands
- **kubectl** - For Kubernetes commands
- **terraform** - For infrastructure commands
- **AWS CLI** - For cloud commands

## Installation Methods

### Method 1: Installation Script (Recommended)
```bash
# Install latest version
./install.sh install

# Install with backup
./install.sh install --backup

# Force reinstall
./install.sh install --force
```

### Method 2: Manual Installation
```bash
# Clone repository
git clone https://github.com/user/bassist.git
cd bassist

# Build from source
cargo build --release

# Install to system
sudo cp target/release/bassist /usr/local/bin/bassist

# Or install to user directory
mkdir -p ~/.local/bin
cp target/release/bassist ~/.local/bin/bassist
```

### Method 3: Cargo Install
```bash
# Install from crates.io (when available)
cargo install bassist

# Install from git repository
cargo install --git https://github.com/user/bassist.git
```

## Installation Options

### Standard Installation
```bash
./install.sh install
```
- Installs to `/usr/local/bin` (system-wide)
- Requires sudo privileges
- Available to all users

### User Installation
```bash
./install.sh install --user
```
- Installs to `~/.local/bin`
- No sudo required
- Available only to current user

### Development Installation
```bash
./install.sh install --dev
```
- Installs from local source
- Uses current directory
- For development/testing

### Force Installation
```bash
./install.sh install --force
```
- Overwrites existing installation
- Creates backup automatically
- For recovery/reinstallation

## Post-Installation Setup

### Verify Installation
```bash
# Check version
bassist --version

# Test basic functionality
bassist -s "git status"

# Show help
bassist --help
```

### Initialize Configuration
```bash
# Create configuration directory
bassist init

# Import shell aliases
bassist import-aliases

# Sync aliases with shell
bassist sync-aliases
```

### PATH Configuration
If bAssist is not found in PATH:

#### For Bash
```bash
# Add to ~/.bashrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### For Zsh
```bash
# Add to ~/.zshrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### For Fish
```bash
# Add to ~/.config/fish/config.fish
echo 'set -x PATH $HOME/.local/bin $PATH' >> ~/.config/fish/config.fish
```

## Configuration

### Configuration Directory
```
~/.bassist/
├── config.json          # Main configuration
├── commands.json         # Command database
├── aliases.json          # User aliases
├── history.json          # Command history
└── backups/             # Installation backups
```

### Default Configuration
```json
{
  "version": "0.1.0",
  "theme": "minimal",
  "safety_level": "caution",
  "auto_backup": true,
  "context_aware": true
}
```

### Environment Variables
```bash
# Optional: Set custom config directory
export BASSIST_CONFIG="$HOME/.config/bassist"

# Optional: Set log level
export BASSIST_LOG="info"

# Optional: Disable safety warnings
export BASSIST_SAFETY="off"
```

## Troubleshooting

### Common Issues

#### Permission Denied
```bash
# Solution 1: Use sudo
sudo ./install.sh install

# Solution 2: Install to user directory
./install.sh install --user
```

#### Rust Not Found
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### Command Not Found
```bash
# Check installation
which bassist

# Check PATH
echo $PATH

# Reinstall if missing
./install.sh install --force
```

#### Build Fails
```bash
# Update Rust
rustup update

# Clean build
cargo clean

# Try again
./install.sh install
```

#### Network Issues
```bash
# Install offline (if source available)
./install.sh install --dev

# Or use local installation
cargo build --release
sudo cp target/release/bassist /usr/local/bin/bassist
```

### Debug Mode
```bash
# Enable debug logging
export RUST_LOG=debug
bassist -s "git status"
```

### Clean Installation
```bash
# Complete cleanup
./install.sh uninstall
rm -rf ~/.bassist

# Fresh install
./install.sh install
```

## Verification

### Basic Tests
```bash
# Test search functionality
bassist -s "git add"

# Test execution (dry run)
bassist -x "git status" --dry-run

# Test alias management
bassist alias add "test" "echo test"
bassist alias list

# Test safety features
bassist --best-practice "rm -rf"
```

### Integration Tests
```bash
# Test with different tools
bassist -s "kubectl get pods"
bassist -s "terraform plan"
bassist -s "docker run"

# Test context awareness
cd ~/project && bassist -s "deploy"
```

## Uninstallation

See [UNINSTALL.md](UNINSTALL.md) for complete uninstallation instructions.

## Upgrading

See [UPGRADE.md](UPGRADE.md) for upgrade instructions.

## Rollback

See [ROLLBACK.md](ROLLBACK.md) for rollback procedures.

## Advanced Installation

### Custom Installation Directory
```bash
# Install to custom directory
INSTALL_DIR="/opt/bassist" ./install.sh install
```

### Systemd Service (optional)
```bash
# Create systemd service for background tasks
sudo tee /etc/systemd/user/bassist.service > /dev/null <<EOF
[Unit]
Description=bAssist Terminal Assistant
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/bassist daemon
Restart=on-failure

[Install]
WantedBy=default.target
EOF

# Enable service
systemctl --user enable bassist
systemctl --user start bassist
```

### Docker Installation
```bash
# Build Docker image
docker build -t bassist .

# Run in container
docker run -it --rm bassist
```

### Nix Installation
```bash
# Using Nix (if available)
nix-env -iA nixpkgs.bassist
```

## Performance Optimization

### Build Optimization
```bash
# Optimized build
cargo build --release --target x86_64-unknown-linux-musl

# Smaller binary
strip target/release/bassist
upx --best target/release/bassist
```

### Runtime Optimization
```bash
# Set environment variables for performance
export RUST_BACKTRACE=0
export RUST_LOG=error
```

## Support

### Getting Help
```bash
# Command help
bassist --help

# Specific command help
bassist alias --help
bassist -s --help
```

### Issue Reporting
- Check [GitHub Issues](https://github.com/user/bassist/issues)
- Include system information
- Provide error logs
- Describe reproduction steps

### Community
- [Discussions](https://github.com/user/bassist/discussions)
- [Wiki](https://github.com/user/bassist/wiki)
- [Discord](https://discord.gg/bassist) (if available)

## Installation Script Options

### Full Command Reference
```bash
./install.sh [COMMAND] [OPTIONS]

Commands:
  install      Install bAssist (default)
  update       Update to latest version
  uninstall    Uninstall bAssist
  rollback     Rollback to previous version
  backup       Create backup of current installation
  restore      Restore from backup
  check        Check for updates
  help         Show this help message

Options:
  --force      Force installation/upgrade
  --backup     Create backup before installing
  --dev        Install from source (development mode)
  --user       Install to user directory
```

### Examples
```bash
# Standard installation
./install.sh install

# Installation with backup
./install.sh install --backup

# Force reinstall
./install.sh install --force

# User installation
./install.sh install --user

# Development installation
./install.sh install --dev

# Update with backup
./install.sh update --backup

# Check for updates
./install.sh check

# Create backup
./install.sh backup

# Rollback
./install.sh rollback

# Uninstall
./install.sh uninstall
```
