{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = with pkgs; [
    # GTK development
    gtk4
    pkg-config
    gcc

    # Wayland support
    wayland
    wayland-protocols
    libxkbcommon

    # Additional libraries that GTK4 commonly needs
    glib
    cairo
    pango
    gdk-pixbuf
    json-glib # For JSON parsing

    # Development tools
    gdb
  ];

  env = {
    PKG_CONFIG_PATH = "${pkgs.gtk4.dev}/lib/pkgconfig:${pkgs.wayland.dev}/lib/pkgconfig";
    # Ensure Wayland backend is preferred
    GDK_BACKEND = "wayland";
  };

  languages.c = {
    enable = true;
  };
}
