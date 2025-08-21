{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-compat.url = "github:edolstra/flake-compat";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    fenix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = nixpkgs.lib.systems.flakeExposed;

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        rust-toolchain = fenix.packages.${system}.complete.toolchain;
      in {
        treefmt = {
          projectRootFile = "flake.lock";
          programs.nixfmt.enable = true;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            rust-toolchain
            pkgs.pkg-config
            pkgs.openssl
          ];
        };
      };
    };
}
