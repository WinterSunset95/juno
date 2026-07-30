{
  description = "Juno's wayland compositor dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    pythonEnv = pkgs.python3.withPackages (ps: with ps; [
      grip
      google-generativeai
      playwright
    ]);
    aider-full = pkgs.aider-chat.overrideAttrs (old: {
      propagatedBuildInputs = old.propagatedBuildInputs ++ (with pkgs.python3Packages; [
        google-generativeai
        playwright
      ]);
    });

  in {
    devShells.${system}.default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        pkg-config
        rustc
        cargo
        rustfmt
        rust-analyzer
      ];

      buildInputs = with pkgs; [
        pythonEnv
        wayland
        wayland-protocols
        wayland-scanner
        libxkbcommon
        libinput
        mesa
        seatd
        udev
        pixman
        libdrm
        libglvnd
        libgbm
        aider-full
      ];

      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
        wayland
        libxkbcommon
        libinput
        mesa
        udev
        pixman
        libdrm
        libglvnd
        libgbm
      ]);
    };
  };
}
