{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/default";
    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = { self, flake-parts, rust-overlay, ... }@ inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
      ];
      perSystem = { config, self', pkgs, lib, system, rust-overlay, ... }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
          tailwind = pkgs.nodePackages.tailwindcss.overrideAttrs (oa: {
            plugins = [
              pkgs.nodePackages."@tailwindcss/forms"
            ];
          });
          buildInputs = [
            pkgs.libiconv
          ];
          nativeBuildInputs = with pkgs; [
            rustToolchain
            tailwind
          ] ++ (
            lib.optionals stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.CoreServices
              pkgs.darwin.apple_sdk.frameworks.CoreFoundation
              pkgs.darwin.apple_sdk.frameworks.Foundation
              pkgs.darwin.apple_sdk.frameworks.AppKit
              pkgs.darwin.apple_sdk.frameworks.WebKit
              pkgs.darwin.apple_sdk.frameworks.Cocoa
            ]
          );
        in
        {
          # Apply Rust overlay
          _module.args.pkgs = import self.inputs.nixpkgs {
            inherit system;
            overlays = [ (import self.inputs.rust-overlay) ];
          };

          # Rust package
          packages.default = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            inherit buildInputs nativeBuildInputs;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            SQLX_OFFLINE = "true";
          };

          # Docker image
          packages.docker = pkgs.dockerTools.buildLayeredImage {
            name = cargoToml.package.name;
            tag = cargoToml.package.version;

            contents = [ pkgs.bash pkgs.coreutils pkgs.curl pkgs.vim ];

            config = {
              Cmd = [ "${self'.packages.default}/bin/${cargoToml.package.name}" ];
            };
          };

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            inputsFrom = [
              config.treefmt.build.devShell
            ];
            shellHook = ''
              echo
              echo "🦀 Run 'just <recipe>' to get started 🦀"
              echo
              echo "Please consider installing the git pre-commit hook:"
              echo "  'just install-git-hooks'"
              just
            '';

            # Enable backtrace
            RUST_BACKTRACE = 1;
            # For rust-analyzer 'hover' tooltips to work.
            RUST_SRC_PATH = rustToolchain + /lib/rustlib/src/rust/library;
            # Local development database connection string
            DATABASE_URL = "postgres://postgres:mysecretpassword@localhost:5432/postgres";

            inherit buildInputs;
            nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
              cargo-watch
              flyctl
              just
              nixpacks
              rust-analyzer
            ]);
          };

          # Add your auto-formatters here.
          # cf. https://numtide.github.io/treefmt/
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
              prettier = {
                enable = true;
                excludes = [ "static/**" ];
                # TODO: Add support for tailwindcss, currently not available in nixpkgs
              };
            };
          };
        };
    };
}
