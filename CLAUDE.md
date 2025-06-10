# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Modali is a Vim-like application launcher built with Rust and Iced. It allows users to navigate through commands using single key presses in a hierarchical menu system, providing quick access to applications and commands.

## Build and Development Commands

### Common Development Tasks

```bash
# Run in development mode (with local bindings.json)
just run

# Run tests
just test

# Lint the codebase
just lint

# Format the code
just format

# Check formatting
just format_check

# Build with Nix
nix build

# Enter development environment
nix develop
```

## Project Architecture

Modali is structured as follows:

- **main.rs**: Entry point that sets up the application, handles CLI arguments, and initializes the GUI
- **gui.rs**: Defines the main application GUI and rendering logic using Iced
- **input.rs**: Handles keyboard input and dispatches appropriate actions
- **parser.rs**: Parses configuration files and builds the command tree structure
- **util.rs**: Utility functions for file handling and command execution

The application follows the Iced framework's architecture with:
1. A main application state (`Modali` struct)
2. Messages for handling events (`Message` enum)
3. Update logic to handle state changes
4. View functions to render the UI

## Configuration

Modali uses a JSON configuration file for defining keybindings:

- In development: Looks for `./bindings.json` first, then `~/.config/modali/bindings.json`
- In release: Only looks in `~/.config/modali/bindings.json`

The configuration defines actions in a tree structure with two types:
- **KeyAction**: A direct command bound to a key
- **SubAction**: A submenu containing nested actions

## Testing

Tests use the `insta` crate for snapshot testing. When making changes to the parser or configuration handling, you may need to update snapshots:

```bash
# Run tests and update snapshots if needed
INSTA_UPDATE=1 cargo test
```