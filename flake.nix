{
  description = "A GTK4 Vim-like application launcher";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.05";
    home-manager.url = "github:nix-community/home-manager";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    home-manager, ...
  }: let
    system = "x86_64-linux"; # Or your target system
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation rec {
      pname = "modali";
      version = "0.1.0";

      src = ./.; # Use the current directory as the source

      nativeBuildInputs = [
        pkgs.gcc
        pkgs.pkg-config
      ];

      buildInputs = [
        pkgs.gtk4
        pkgs.json-glib
        # Dependencies like glib, cairo, pango, gdk-pixbuf, libxkbcommon, wayland
        # are pulled in automatically by gtk4 and json-glib.
      ];

      buildPhase = ''
        runHook preBuild
        gcc $NIX_CFLAGS_COMPILE main.c -o modali $(pkg-config --cflags --libs gtk4 gio-2.0 json-glib-1.0)
        runHook postBuild
      '';

      installPhase = ''
        runHook preInstall
        mkdir -p $out/bin
        mkdir -p $out/share/modali

        cp modali $out/bin/
        cp bindings.json $out/share/modali/ # Default bindings
        cp style.css $out/share/modali/
        runHook postInstall
      '';
    };

    apps.${system}.default = {
      type = "app";
      program = "${self.packages.${system}.default}/bin/modali";
    };

    homeManagerModules.default = {
      # Define this module for x86_64-linux, can be adapted for other systems
      # by passing system through or iterating over supportedSystems
      imports = [ home-manager.nixosModules.home-manager ]; # If used in NixOS config
                                                            # For standalone HM, this might not be needed or structured differently
                                                            # Simpler approach for flake module:
      config = { lib, pkgs, config, ... }:
        let
          cfg = config.programs.modali;
          modaliPackage = self.packages.${system}.default;
        in
        {
          options.programs.modali = {
            enable = lib.mkEnableOption "Whether to enable Modali launcher";
            package = lib.mkOption {
              type = lib.types.package;
              default = modaliPackage;
              description = "Modali package to use.";
            };
            keybindings = lib.mkOption {
              type = lib.types.nullOr (lib.types.listOf lib.types.attrs); # Expects a list of attribute sets for the JSON array
              default = null;
              description = ''
                Keybindings configuration for Modali.
                If set, this attribute set will be converted to JSON and written to bindings.json.
                Example: 
                  [
                    { key = "f"; description = "File operations"; sub_actions = [ ... ]; }
                    { key = "p"; command = "firefox"; description = "Launch Firefox"; }
                  ]
                If null, a default bindings.json from the package will be used as a template.
              '';
            };
          };

          config = lib.mkIf cfg.enable {
            home.packages = [ cfg.package ];
          } // lib.mkIf (cfg.enable && cfg.keybindings != null) { # Merge this block if keybindings are provided
            xdg.configFile."modali/bindings.json" = {
              # User has defined keybindings in their Home Manager config
              text = pkgs.formats.json {}.generate "modali-bindings.json" cfg.keybindings;
            };
          };
        };
    };
  };
}
