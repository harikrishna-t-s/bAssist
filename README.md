# bAssist - Terminal Command Assistant

bAssist is a high-performance terminal tool that helps you find and execute commands based on natural language descriptions. Never struggle to remember complex command syntax again!

## What bAssist Does

- **Smart Command Search**: Type what you want to do in plain English (e.g., "add all files to git") and get the exact command
- **Minimal TUI Interface**: Clean, keyboard-driven interface inspired by LazyGit
- **Alias Management**: Create and manage custom command aliases easily
- **Command History**: Track your frequently used commands
- **High Performance**: Instant search results with minimal resource usage

## Quick Start

### Installation (One Command)

```bash
curl -sSL https://raw.githubusercontent.com/user/bassist/main/install.sh | bash
```

### Basic Usage

```bash
# Launch the interactive interface
bassist

# Search for commands directly
bassist -s "add all files to git"

# Add a custom alias
bassist alias add "ll" "ls -la"

# List all aliases
bassist alias list

# Show command history
bassist history
```

## Features

### 🔍 Smart Search
Search for commands using natural language:
- `"add all files to git"` → `git add .`
- `"list all files with details"` → `ls -la`
- `"find files by name"` → `find . -name`
- `"run docker container"` → `docker run`

### 🎯 Interactive Interface
The TUI provides multiple modes:
- **Search Mode**: Type queries and get instant results
- **Browse Mode**: Navigate through all available commands
- **Alias Mode**: Manage your custom aliases
- **History Mode**: View your command history

### ⌨️ Keyboard Controls
- `↑/↓` - Navigate through results
- `Enter` - Execute selected command
- `Tab` - Switch between modes
- `Esc` - Exit
- Type to search (in Search mode)

### 🏷️ Alias Management
Create shortcuts for frequently used commands:
```bash
# Add alias
bassist alias add "gst" "git status"

# Remove alias
bassist alias remove "gst"

# List all aliases
bassist alias list
```

## Command Categories

bAssist comes with pre-built commands for common tasks:

### Git Commands
- Add, commit, push, pull operations
- Branch management
- Status and diff commands

### Docker Commands
- Container management
- Image operations
- Network and volume commands

### System Commands
- File operations (find, ls, cp, mv)
- Process management
- System information

### Network Commands
- Ping, curl, wget
- Port scanning
- Network configuration

## Configuration

bAssist stores its configuration in `~/.bassist/`:
- `commands.json` - Command database
- `aliases.json` - Your custom aliases
- `history.json` - Command history
- `config.json` - Application settings

## Examples

### Finding Git Commands
```bash
bassist -s "commit all changes"
# Returns: git commit -m "message"

bassist -s "push to remote"
# Returns: git push
```

### System Administration
```bash
bassist -s "show disk usage"
# Returns: df -h

bassist -s "find large files"
# Returns: find . -type f -size +100M
```

### Docker Operations
```bash
bassist -s "run nginx container"
# Returns: docker run -d -p 80:80 nginx

bassist -s "list all containers"
# Returns: docker ps -a
```

## Performance

- **Startup Time**: <100ms
- **Search Response**: <50ms
- **Memory Usage**: <10MB
- **Binary Size**: <5MB

## Requirements

- Rust 1.70+ (for installation from source)
- Linux or macOS
- Bash shell

## Installation Methods

### Method 1: Install Script (Recommended)
```bash
curl -sSL https://raw.githubusercontent.com/user/bassist/main/install.sh | bash
```

### Method 2: Manual Installation
```bash
git clone https://github.com/user/bassist.git
cd bassist
cargo build --release
sudo cp target/release/bassist /usr/local/bin/
```

### Method 3: Download Binary
```bash
wget https://github.com/user/bassist/releases/latest/download/bassist
chmod +x bassist
sudo mv bassist /usr/local/bin/
```

## Troubleshooting

### Command Not Found
If you get "command not found" after installation:
```bash
# Add to your shell profile
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Permission Denied
```bash
# Make the binary executable
chmod +x ~/.local/bin/bassist
```

### Update bAssist
```bash
# Re-run the installation script
curl -sSL https://raw.githubusercontent.com/user/bassist/main/install.sh | bash
```

## Contributing

bAssist is open source! Contributions welcome:
- Add new commands to the database
- Improve the matching algorithm
- Fix bugs and improve documentation
- Suggest new features

## License

MIT License - see LICENSE file for details.

## Support

- 📖 Documentation: [GitHub Wiki](https://github.com/user/bassist/wiki)
- 🐛 Bug Reports: [GitHub Issues](https://github.com/user/bassist/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/user/bassist/discussions)

---

**bAssist** - Making terminal commands simple and accessible for everyone.
