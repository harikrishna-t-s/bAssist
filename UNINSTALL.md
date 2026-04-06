# bAssist Uninstallation Guide

## Automatic Uninstallation

### Using the Installation Script
```bash
./install.sh uninstall
```

The uninstall script will:
- Remove the bAssist binary from `/usr/local/bin/bassist` or `~/.local/bin/bassist`
- Prompt to remove configuration directory (`~/.bassist`)
- Prompt to remove backup directory (`~/.bassist/backups`)
- Clean up all related files

## Manual Uninstallation

### Step 1: Remove Binary
```bash
# Remove from system directory
sudo rm /usr/local/bin/bassist

# Or remove from user directory
rm ~/.local/bin/bassist
```

### Step 2: Remove Configuration (Optional)
```bash
# Remove configuration directory
rm -rf ~/.bassist
```

### Step 3: Remove from PATH (if added)
If you added bAssist to your PATH manually, edit your shell configuration file:
```bash
# Remove this line from ~/.bashrc or ~/.zshrc
export PATH="$HOME/.local/bin:$PATH"
```

### Step 4: Remove Shell Aliases (if created)
If you created any bAssist aliases, remove them from your shell configuration:
```bash
# Remove from ~/.bashrc or ~/.zshrc
alias bassist='~/.local/bin/bassist'
```

### Step 5: Reload Shell
```bash
source ~/.bashrc
# or
source ~/.zshrc
```

## Verification

Verify uninstallation:
```bash
which bassist
# Should return: bassist not found

bassist --help
# Should return: command not found
```

## Clean Uninstallation

To completely remove all traces of bAssist:

```bash
# Remove binary
sudo rm -f /usr/local/bin/bassist
rm -f ~/.local/bin/bassist

# Remove all data
rm -rf ~/.bassist

# Remove from shell configurations
sed -i '/bassist/d' ~/.bashrc
sed -i '/bassist/d' ~/.zshrc

# Reload shell
source ~/.bashrc
```

## Troubleshooting

### Permission Denied
If you get permission errors:
```bash
sudo ./install.sh uninstall
```

### File Not Found
If the script can't find bAssist:
```bash
# Find where bassist is installed
which bassist
find /usr/local -name bassist 2>/dev/null
find ~ -name bassist 2>/dev/null
```

### Configuration Directory Remains
If the configuration directory persists:
```bash
# Force remove
sudo rm -rf ~/.bassist
```

## Reinstallation

If you want to reinstall bAssist after uninstalling:

```bash
# Clean installation
./install.sh install --force

# Or with backup
./install.sh install --backup
```

## Backup and Restore

Before uninstalling, you may want to backup your configuration:

```bash
# Backup configuration
cp -r ~/.bassist ~/.bassist_backup

# Restore after reinstallation
cp -r ~/.bassist_backup/* ~/.bassist/
```
