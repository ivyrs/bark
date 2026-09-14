{ pkgs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rust-analyzer"
      "rustfmt"
    ];
  };

  packages = with pkgs; [
    pkg-config
    gcc

    gtk4
    gtk4-layer-shell

    bacon
    cargo-nextest
    watchexec
    just

    nixfmt # for devenv formatting
  ];

  git-hooks.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };

  scripts.watch.exec = ''
    bacon clippy
  '';
}
