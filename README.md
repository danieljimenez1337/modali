# Modali Launcher

A GTK4 Vim-like application launcher.

## Installation & Configuration with Nix and Home Manager

This project provides a Nix Flake for easy building, development, and integration with Home Manager.

### Prerequisites

1.  **Nix with Flakes Enabled**: Ensure you have Nix installed and the `flakes` experimental feature enabled.
    (Add `experimental-features = nix-command flakes` to your `nix.conf`.)
2.  **Home Manager**: You should have Home Manager installed and configured to manage your user environment.

### Using the Modali Home Manager Module

1.  **Add Modali Flake as an Input**:
    In your Home Manager flake (e.g., `~/your-home-manager-config/flake.nix`), add this Modali project as an input:

    ```nix
    {
      inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; # Or your preferred channel
        home-manager = {
          url = "github:nix-community/home-manager";
          inputs.nixpkgs.follows = "nixpkgs";
        };
        # Add Modali launcher flake
        modali-launcher = {
          url = "path:/path/to/your/modali/project"; # Or github:your-username/modali if published
          # If modali-launcher also needs nixpkgs, and you want it to use the same one:
          # inputs.nixpkgs.follows = "nixpkgs"; 
        };
        # ... other inputs
      };

      outputs = { self, nixpkgs, home-manager, modali-launcher, ... }:
        # ... your Home Manager outputs structure
    }
    ```

2.  **Import and Configure the Modali Module**:
    In your `home.nix` (or a file imported by it where you define Home Manager configurations for your user), import the Modali module and configure it:

    ```nix
    {
      # ... other arguments like pkgs, config, lib, etc.
      # Make sure 'modali-launcher' input is available here if you're passing inputs through
    }:
    {
      imports = [
        # If modali-launcher is an input from your HM flake:
        modali-launcher.homeManagerModules.default 
        # ... other imports
      ];

      programs.modali = {
        enable = true;
        # Define your keybindings directly in Nix.
        # This will generate ~/.config/modali/bindings.json
        keybindings = [
          {
            key = "g";
            description = "Git Commands";
            sub_actions = [
              { key = "s"; command = "git status"; description = "Status"; }
              { key = "c"; command = "git commit"; description = "Commit"; }
              { key = "p"; command = "git push"; description = "Push"; }
            ];
          }
          {
            key = "b";
            description = "Browser";
            command = "firefox"; # Or your preferred browser
          }
          {
            key = "t";
            description = "Terminal";
            command = "kitty --single-instance"; # Or your preferred terminal
          }
          {
            key = "e";
            description = "Editor";
            command = "emacsclient -c -a emacs";
          }
          {
            key = "s";
            description = "System";
            sub_actions = [
              { key = "l"; command = "loginctl lock-session"; description = "Lock Screen"; }
              { key = "r"; command = "systemctl reboot"; description = "Reboot"; }
              { key = "p"; command = "systemctl poweroff"; description = "Power Off"; }
            ];
          }
        ];
      };

      # Example: Bind Modali to a keyboard shortcut using your window manager's tools
      # This part is external to Modali itself. For example, in Sway/i3:
      # services.sway.config.keybindings = {
      #   "Mod4+space" = "exec modali"; 
      # };
      # Or for other desktop environments, use their respective shortcut configuration tools.
    }
    ```

3.  **Rebuild Home Manager**:
    Apply the configuration:
    ```bash
    home-manager switch --flake ~/your-home-manager-config#your-username
    ```

    Modali will now be installed, and `~/.config/modali/bindings.json` will be created based on your `programs.modali.keybindings` definition.

### Development

If you want to work on Modali itself:

1.  **Clone the repository.**
2.  **Enter the development shell:**
    ```bash
    nix develop
    ```
    This provides all necessary dependencies (GTK4, JSON-GLib, GCC, Pkg-config, GDB).
3.  **Build and run:**
    Inside the shell, you can compile and run Modali:
    ```bash
    make
    ./modali
    ```
    For development, Modali will try to load `bindings.json` and `style.css` from the current directory if it cannot find them in the XDG config path or relative to the executable's share directory. You can keep copies in the project root for easy testing.

### Manual Build (without Home Manager installation)

Build the package:
```bash
nix build .#
```
This creates a `./result` symlink. The application is at `./result/bin/modali`.

Run the application:
```bash
nix run .#
```
When run this way, it will look for `bindings.json` in `~/.config/modali/bindings.json` and `style.css` in its Nix store share path.
