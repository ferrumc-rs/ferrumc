{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      flake-parts,
      nixpkgs,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem =
        {
          pkgs,
          system,
          ...
        }:
        {
          formatter = nixpkgs.legacyPackages.${system}.nixfmt-rfc-style;
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
              (self: super: {
                rustToolchain =
                  let
                    rust = super.rust-bin;
                  in
                  if builtins.pathExists ./rust-toolchain.toml then
                    rust.fromRustupToolchainFile ./rust-toolchain.toml
                  else if builtins.pathExists ./rust-toolchain then
                    rust.fromRustupToolchainFile ./rust-toolchain
                  else
                    rust.nightly.latest.default;
              })
            ];
            config = { };
          };

          # Used to check formatting for nix specificly
          checks.fmt-check =
            pkgs.runCommand "format-check"
              {
                src = ./.;
                doCheck = true;
                nativeBuildInputs = [
                  pkgs.nixfmt-rfc-style
                ];
              }
              ''
                					nixfmt --check .
                					touch $out
                				'';

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              pkg-config
              openssl
            ];
          };

        };
    };
}
