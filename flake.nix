{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    # Necessary until we have a _simple_ way to make a naked shell (no stdenv env vars)
    devshell.url = "github:numtide/devshell";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ flake-parts, devshell, systems, ... }:
  flake-parts.lib.mkFlake { inherit inputs; } {
    imports = [ devshell.flakeModule ];
    systems = import systems;

    perSystem = { pkgs, ... }: {
      devshells.default = {
        packages = with pkgs; [
          cargo
          rustc # necessary somehow to get good LSP suggestions 🤔
          gcc
        ];
      };
    };
  };
}
