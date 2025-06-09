{
  description = "A GTK4 Vim-like application launcher";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.05";
    home-manager.url = "github:nix-community/home-manager";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = {
    self,
    nixpkgs,
    home-manager,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = "modali";
      version = "0.1.0";
      src = ./.;
      cargoLock = {
        lockFile = ./Cargo.lock;
      };
      nativeBuildInputs = with pkgs; [
        pkg-config
        rust-bin.nightly.latest.default
        makeWrapper  # Add this for wrapping the binary
      ];
      buildInputs = with pkgs; [
        gtk4
        json-glib
        libxkbcommon
        vulkan-loader
        wayland  # Runtime library needed for iced_layershell
      ];

      # Wrap the binary to set LD_LIBRARY_PATH for Wayland
      postInstall = ''
        wrapProgram $out/bin/modali \
          --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath [ pkgs.wayland pkgs.libxkbcommon pkgs.vulkan-loader ]}
      '';

      meta = with pkgs.lib; {
        description = "A GTK4 Vim-like application launcher";
        homepage = "https://github.com/your-username/modali";
        license = licenses.mit;
        maintainers = [];
        platforms = platforms.linux;
      };
    };

    apps.${system}.default = {
      type = "app";
      program = "${self.packages.${system}.default}/bin/modali";
    };

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rust-bin.nightly.latest.default
        pkg-config
        gtk4
        json-glib
        wayland
        wayland.dev  # Add the development headers
        libxkbcommon
        vulkan-loader
        cargo-insta
      ];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
        pkgs.wayland
        pkgs.vulkan-loader
        pkgs.libxkbcommon
      ];
    };

    homeManagerModules.default = {
      config,
      lib,
      pkgs,
      ...
    }: let
      cfg = config.programs.modali;
      modaliPackage = self.packages.${system}.default;
    in {
      options.programs.modali = {
        enable = lib.mkEnableOption "Enable the Modali vim-like launcher";
        keybindings = lib.mkOption {
          type = with lib.types; listOf attrs;
          default = [];
          description = "Keybindings for Modali, as a list of attribute sets.";
        };
      };
      config = lib.mkIf cfg.enable {
        home.packages = [modaliPackage];
        xdg.configFile."modali/bindings.json".text = builtins.toJSON cfg.keybindings;
      };
    };
  };
}
