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

    homeManagerModules.default = { config, lib, pkgs, ... }:
      let
        cfg = config.programs.modali;
        modaliPackage = self.packages.${system}.default;
      in
      {
        options.programs.modali = {
          enable = lib.mkEnableOption "Enable the Modali vim-like launcher";
          keybindings = lib.mkOption {
            type = with lib.types; listOf attrs;
            default = [];
            description = "Keybindings for Modali, as a list of attribute sets.";
          };
        };

        config = lib.mkIf cfg.enable {
          home.packages = [ modaliPackage ];
          xdg.configFile."modali/bindings.json".text =
            pkgs.formats.json {}.generate "modali-bindings.json" cfg.keybindings;
        };
      };

  };
}
