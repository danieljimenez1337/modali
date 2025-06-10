# Modali

A Vim-like application launcher built with Rust and Iced.

## Features

- **Vim-like keybindings**: Navigate through commands using single key presses
- **Hierarchical commands**: Organize commands in nested menus
- **Iced + Wayland**: Modern UI with proper Wayland support
- **XDG compliant**: Follows XDG Base Directory specification
- **Nix integration**: Easy installation and configuration with Nix and Home Manager

## Installation

### With Nix Flakes and Home Manager

1. Add Modali as an input to your flake:

```nix
inputs.modali.url = "github:danieljimenez1337/modali";
```

2. Import the module and enable Modali in your Home Manager configuration:

```nix
{ config, pkgs, inputs, ... }:
{
  imports = [
    inputs.modali.homeManagerModules.default
  ];

  programs.modali = {
    enable = true;
    keybindings = [
      {
        type = "KeyAction";
        key = "f";
        description = "Firefox";
        command = "firefox";
      }
      {
        type = "KeyAction";
        key = "t";
        description = "Terminal";
        command = "kitty";
      }
      {
        type = "SubAction";
        key = "s";
        description = "System";
        sub_actions = [
          {
            type = "KeyAction";
            key = "l";
            description = "Lock Screen";
            command = "loginctl lock-session";
          }
          {
            type = "KeyAction";
            key = "r";
            description = "Reboot";
            command = "systemctl reboot";
          }
          {
            type = "KeyAction";
            key = "p";
            description = "Power Off";
            command = "systemctl poweroff";
          }
        ];
      }
    ];
  };
}
```

3. Apply your configuration:

```bash
home-manager switch --flake .#your-username
```

### Standalone Installation

Build and run with Nix:

```bash
nix run github:danieljimenez1337/modali
```

Or build locally:

```bash
nix build
./result/bin/modali
```

## Configuration

Modali looks for its configuration file differently depending on how it's built:

**Debug builds** (development with `cargo run`):
1. `./bindings.json` (current directory)
2. `~/.config/modali/bindings.json` (XDG config directory)

**Release builds** (installed via Nix/Home Manager):
- `~/.config/modali/bindings.json` (XDG config directory only)

This ensures that developers can use a local config file for testing, while installed versions follow XDG conventions.

### Configuration Format

The `bindings.json` file uses the following structure:

```json
[
  {
    "type": "KeyAction",
    "key": "f",
    "description": "Firefox",
    "command": "firefox"
  },
  {
    "type": "KeyAction", 
    "key": "t",
    "description": "Terminal",
    "command": "kitty"
  },
  {
    "type": "SubAction",
    "key": "g",
    "description": "Git Commands",
    "sub_actions": [
      {
        "type": "KeyAction",
        "key": "s",
        "description": "Git Status", 
        "command": "git status"
      },
      {
        "type": "KeyAction",
        "key": "c",
        "description": "Git Commit",
        "command": "git commit"
      }
    ]
  }
]
```

#### Action Types

- **KeyAction**: A single command bound to a key
  - `type`: "KeyAction"
  - `key`: Single character key
  - `description`: Human-readable description
  - `command`: Shell command to execute

- **SubAction**: A submenu with nested actions
  - `type`: "SubAction"  
  - `key`: Single character key
  - `description`: Human-readable description
  - `sub_actions`: Array of nested actions

## Usage

1. Launch Modali (bind it to a keyboard shortcut in your window manager)
2. Type keys to navigate through the command tree
3. Press the final key to execute a command
4. Press Escape to close without executing

### Example Navigation

With the configuration above:
- Press `f` → Launch Firefox
- Press `g` → Show Git submenu
- Press `g`, then `s` → Execute `git status`

## Development

### Prerequisites

- Rust (nightly)
- GTK4 development libraries
- Wayland development libraries

### With Nix

Enter the development shell:

```bash
nix develop
```

Build and run:

```bash
cargo run
```

### Manual Setup

Install dependencies:

```bash
# On Ubuntu/Debian
sudo apt install libgtk-4-dev libwayland-dev libxkbcommon-dev

# On Arch
sudo pacman -S gtk4 wayland libxkbcommon
```

Build and run:

```bash
cargo build --release
./target/release/modali
```

## Dependencies

- **iced**: Cross-platform GUI framework
- **iced_layershell**: Wayland layer shell support for Iced
- **serde**: JSON serialization/deserialization
- **Iced**: Cross-platform GUI framework
- **Wayland**: Display server protocol

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]