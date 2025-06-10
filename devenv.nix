{
  pkgs,
  lib,
  # config,
  # inputs,
  ...
}: {
  packages = with pkgs; [
    wayland
    libxkbcommon

    vulkan-loader

    # Development tools
    cargo-insta
    just
  ];

  env.LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${lib.makeLibraryPath [pkgs.wayland pkgs.vulkan-loader pkgs.libxkbcommon]}";

  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustchannel
    channel = "nightly";

    components = ["rustc" "cargo" "clippy" "rustfmt" "rust-analyzer"];
  };
}
