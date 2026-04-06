# bAssist Upgrade Guide

## Automatic Upgrade

### Using the Installation Script
```bash
./install.sh update
```

The update script will:
- Check for the latest version
- Create a backup of the current installation
- Download and build the latest version
- Install the new version
- Verify the upgrade

### Check for Updates Only
```bash
./install.sh check
```

## Manual Upgrade

### Step 1: Backup Current Installation
```bash
./install.sh backup
```

### Step 2: Download Latest Source
```bash
# Clone the latest version
git clone https://github.com/user/bassist.git bassist_new
cd bassist_new
```

### Step 3: Build and Install
```bash
# Build the project
cargo build --release

# Install to system
sudo cp target/release/bassist /usr/local/bin/bassist
```

### Step 4: Verify Installation
```bash
bassist --version
```

## Version Management

### Check Current Version
```bash
bassist --version
```

### Check Available Versions
```bash
# Using GitHub API
curl -s https://api.github.com/repos/user/bassist/releases | grep '"tag_name"' | head -5

# Using git tags
git ls-remote --tags https://github.com/user/bassist.git
```

### Install Specific Version
```bash
# Clone specific tag
git clone --branch v0.1.0 https://github.com/user/bassist.git
cd bassist
cargo build --release
sudo cp target/release/bassist /usr/local/bin/bassist
```

## Rollback

### Automatic Rollback
```bash
./install.sh rollback
```

### Manual Rollback
```bash
# List available backups
ls -la ~/.bassist/backups/

# Restore specific backup
sudo cp ~/.bassist/backups/bassist_backup_YYYYMMDD_HHMMSS /usr/local/bin/bassist
```

## Upgrade Strategies

### Safe Upgrade (Recommended)
```bash
# Create backup before upgrade
./install.sh install --backup

# Test new version
bassist --version
bassist -s "git status"

# Rollback if needed
./install.sh rollback
```

### Force Upgrade
```bash
# Force reinstall (use with caution)
./install.sh install --force
```

### Development Upgrade
```bash
# Install from current source
./install.sh install --dev
```

## Troubleshooting

### Update Fails
If the update fails:
```bash
# Check network connectivity
curl -I https://github.com

# Check Rust installation
rustc --version
cargo --version

# Clean build and retry
cargo clean
./install.sh update
```

### Version Conflicts
If you have version conflicts:
```bash
# Completely remove old version
./install.sh uninstall

# Clean install new version
./install.sh install
```

### Permission Issues
If you get permission errors:
```bash
# Use sudo for system installation
sudo ./install.sh update

# Or install to user directory
./install.sh update --user
```

### Build Errors
If the build fails:
```bash
# Update Rust toolchain
rustup update

# Clean build cache
cargo clean

# Check dependencies
cargo check

# Try again
./install.sh update
```

## Configuration Migration

### Backup Configuration
```bash
cp -r ~/.bassist ~/.bassist_backup_$(date +%Y%m%d)
```

### Migrate Configuration
Most configuration files are compatible across versions, but check for:
- New command database format
- Updated configuration schema
- New features requiring configuration

```bash
# Test configuration with new version
bassist -s "test"
```

### Reset Configuration
If configuration causes issues:
```bash
# Reset to defaults
rm -rf ~/.bassist
bassist init
```

## Release Channels

### Stable Release
```bash
./install.sh install
```

### Development Release
```bash
./install.sh install --dev
```

### Beta Release
```bash
# Install specific beta version
git clone --branch v0.2.0-beta https://github.com/user/bassist.git
```

## Automated Upgrades

### Cron Job Setup
```bash
# Add to crontab for weekly updates
echo "0 2 * * 0 /path/to/install.sh update" | crontab -
```

### Update Notification
```bash
# Create a script to check for updates
cat > check_bassist_update.sh << 'EOF'
#!/bin/bash
./install.sh check | grep -q "Update available" && echo "bAssist update available"
EOF

chmod +x check_bassist_update.sh
```

## Upgrade Best Practices

1. **Always backup before upgrading**
2. **Test new version in safe environment**
3. **Check release notes for breaking changes**
4. **Verify configuration compatibility**
5. **Keep rollback option available**
6. **Monitor for issues after upgrade**

## Release Notes

### Version 0.1.0 → 0.2.0
- Enhanced safety features
- New DevOps command database
- Improved context detection
- Better error handling

### Version 0.2.0 → 0.3.0 (Planned)
- Cloud provider integration
- Advanced command templates
- Performance improvements
- Enhanced TUI interface

## Support

If you encounter issues during upgrade:

1. Check the [troubleshooting section](#troubleshooting)
2. Review [GitHub issues](https://github.com/user/bassist/issues)
3. Create a new issue with details
4. Include system information and error logs

```bash
# Collect system information for bug reports
echo "System Info:"
uname -a
echo "Rust Info:"
rustc --version
echo "bAssist Info:"
bassist --version 2>/dev/null || echo "Not installed"
```
