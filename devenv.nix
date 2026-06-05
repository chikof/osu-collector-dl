{
  pkgs,
  lib,
  ...
}: {
  packages =
    [
      pkgs.sccache
      pkgs.libxkbcommon
      pkgs.libGL
      pkgs.wayland
    ]
    ++ lib.optionals pkgs.stdenv.isLinux [
      pkgs.libXcursor
      pkgs.libXrandr
      pkgs.libXi
      pkgs.libX11
    ];

  env.RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  env.LD_LIBRARY_PATH = lib.makeLibraryPath [
    pkgs.libxkbcommon
    pkgs.libGL
    pkgs.wayland
  ];

  dotenv.enable = true;
  languages.rust = {
    enable = true;
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  processes = {
    sccache.exec = "sccache --start-server";
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    alejandra.enable = true;
  };
}
