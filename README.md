# rofi-linear

A rofi plugin for quickly creating Linear issues from anywhere on your desktop.

## Installation

### From source

```bash
git clone https://github.com/yourusername/rofi-linear
cd rofi-linear
make install
```

### Arch Linux (AUR)

```bash
yay -S rofi-linear
```

## Usage

### Initial Setup

1. **Authenticate with Linear:**
   ```bash
   rofi-linear auth
   ```
   This opens your browser to create an API key.

2. **Link a team:**
   ```bash
   rofi-linear link
   ```

### Creating Issues

```bash
# Default flow - prompts for title and description
rofi-linear run

# Quick mode - title only
rofi-linear run -q

# Skip opening browser after creation
rofi-linear run -s

# Specify a team
rofi-linear run work
```

### Managing Teams

```bash
# List linked teams
rofi-linear list

# Unlink a team
rofi-linear unlink work
```

## Sway/i3 Keybinding

Add to your `~/.config/sway/config` or `~/.config/i3/config`:

```bash
# Linear issue creation
bindsym $mod+i exec rofi-linear run
```

## Configuration

Config files are stored in `~/.config/rofi-linear/`:

- `config.yaml` - Team configuration
- `creds.yaml` - API key (git-ignored)

## License

MIT
