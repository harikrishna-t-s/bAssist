# bAssist Rollback Guide

## Automatic Rollback

### Using the Installation Script
```bash
./install.sh rollback
```

The rollback script will:
- Find the most recent backup
- Restore the binary from backup
- Verify the rollback
- Report success/failure

### Manual Rollback
```bash
# List available backups
ls -la ~/.bassist/backups/

# Restore specific backup
sudo cp ~/.bassist/backups/bassist_backup_YYYYMMDD_HHMMSS /usr/local/bin/bassist
```

## Backup Management

### Create Backup
```bash
./install.sh backup
```

### List Backups
```bash
ls -la ~/.bassist/backups/
```

### Backup Naming Convention
Backups are named using the format:
```
bassist_backup_YYYYMMDD_HHMMSS
```

Example:
```
bassist_backup_20240405_143022
```

### Check Last Backup
```bash
cat ~/.bassist/backups/last_backup
```

## Rollback Scenarios

### Scenario 1: Update Failed
```bash
# Update failed, rollback to previous version
./install.sh rollback

# Verify rollback
bassist --version
```

### Scenario 2: New Version Issues
```bash
# New version has bugs, rollback
./install.sh rollback

# Test rollback worked
bassist -s "git status"
```

### Scenario 3: Configuration Issues
```bash
# Rollback binary but keep config
./install.sh rollback

# Reset configuration if needed
rm ~/.bassist/config.json
bassist init
```

## Advanced Rollback

### Rollback to Specific Version
```bash
# Find backup timestamp
ls -la ~/.bassist/backups/

# Restore specific backup
sudo cp ~/.bassist/backups/bassist_backup_20240405_120000 /usr/local/bin/bassist
```

### Rollback with Configuration
```bash
# Backup current config
cp ~/.bassist/config.json ~/.bassist/config_backup_$(date +%Y%m%d)

# Rollback binary
./install.sh rollback

# Test with old config
bassist -s "test"
```

### Multiple Rollbacks
```bash
# If first rollback fails, try previous backup
sudo cp ~/.bassist/backups/bassist_backup_20240405_110000 /usr/local/bin/bassist
```

## Troubleshooting

### No Backup Found
```bash
# Error: No backup found
./install.sh rollback

# Solution: Reinstall from source
./install.sh install --force
```

### Backup Corrupted
```bash
# Test backup integrity
file ~/.bassist/backups/bassist_backup_*

# If corrupted, remove and reinstall
rm ~/.bassist/backups/bassist_backup_corrupted
./install.sh install --force
```

### Permission Issues
```bash
# Permission denied during rollback
sudo ./install.sh rollback

# Or fix permissions
sudo chown $USER:$USER ~/.bassist/backups/*
chmod +x ~/.bassist/backups/*
```

### Binary Not Working After Rollback
```bash
# Test binary
/usr/local/bin/bassist --version

# If broken, try different backup
sudo cp ~/.bassist/backups/bassist_backup_YYYYMMDD_HHMMSS /usr/local/bin/bassist

# Or reinstall
./install.sh install --force
```

## Backup Cleanup

### List All Backups
```bash
ls -la ~/.bassist/backups/ | grep bassist_backup
```

### Remove Old Backups
```bash
# Keep only last 3 backups
cd ~/.bassist/backups/
ls -t bassist_backup_* | tail -n +4 | xargs rm -f
```

### Clean All Backups
```bash
# Remove all backups
rm -rf ~/.bassist/backups/
```

### Backup Size Check
```bash
# Check backup directory size
du -sh ~/.bassist/backups/

# Check individual backup sizes
ls -lh ~/.bassist/backups/bassist_backup_*
```

## Rollback Best Practices

### Before Update
```bash
# Always create backup before update
./install.sh backup
./install.sh update
```

### After Update
```bash
# Test new version immediately
bassist --version
bassist -s "git status"

# Rollback quickly if issues
./install.sh rollback
```

### Regular Backup Schedule
```bash
# Create backup weekly
echo "0 2 * * 0 /path/to/install.sh backup" | crontab -
```

## Emergency Procedures

### Complete System Restore
```bash
# If bAssist completely broken
./install.sh uninstall
./install.sh install --force
```

### Configuration Reset
```bash
# Reset to defaults
rm -rf ~/.bassist
bassist init
```

### Manual Binary Restore
```bash
# If script fails, manual restore
sudo cp ~/.bassist/backups/bassist_backup_YYYYMMDD_HHMMSS /usr/local/bin/bassist
sudo chmod +x /usr/local/bin/bassist
```

## Rollback Validation

### Verify Binary
```bash
# Check binary works
bassist --version

# Check basic functionality
bassist -s "git status"
```

### Verify Configuration
```bash
# Check config loaded
bassist alias list

# Check command database
bassist -s "test"
```

### Verify Integration
```bash
# Check PATH
which bassist

# Check shell integration
bassist --help
```

## Rollback Automation

### Automated Rollback Script
```bash
cat > rollback_bassist.sh << 'EOF'
#!/bin/bash

# Automated rollback with validation

echo "Starting bAssist rollback..."

# Create backup of current (broken) version
if [ -f /usr/local/bin/bassist ]; then
    cp /usr/local/bin/bassist ~/.bassist/backups/bassist_broken_$(date +%Y%m%d_%H%M%S)
fi

# Rollback to last good version
./install.sh rollback

# Validate rollback
if bassist --version > /dev/null 2>&1; then
    echo "Rollback successful!"
    bassist --version
else
    echo "Rollback failed! Manual intervention required."
    exit 1
fi
EOF

chmod +x rollback_bassist.sh
```

### Health Check Script
```bash
cat > health_check.sh << 'EOF'
#!/bin/bash

# Check bAssist health after rollback

echo "Checking bAssist health..."

# Basic checks
if ! command -v bassist &> /dev/null; then
    echo "ERROR: bassist not found in PATH"
    exit 1
fi

if ! bassist --version &> /dev/null; then
    echo "ERROR: bassist --version failed"
    exit 1
fi

if ! bassist -s "git status" &> /dev/null; then
    echo "ERROR: basic search failed"
    exit 1
fi

echo "All health checks passed!"
EOF

chmod +x health_check.sh
```

## Documentation

### Rollback Log
```bash
# Create rollback log
echo "$(date): Rolled back to version $(bassist --version)" >> ~/.bassist/rollback.log
```

### Issue Tracking
```bash
# Log issues that caused rollback
echo "$(date): Issue description - Rolled back" >> ~/.bassist/issues.log
```

## Support

If rollback fails:

1. Check [troubleshooting section](#troubleshooting)
2. Try manual restore methods
3. Reinstall as last resort
4. Report issue with details

```bash
# Collect info for support
echo "Rollback Info:"
echo "Current time: $(date)"
echo "Last backup: $(cat ~/.bassist/backups/last_backup 2>/dev/null || echo 'None')"
echo "Available backups:"
ls -la ~/.bassist/backups/ 2>/dev/null || echo "No backup directory"
```
