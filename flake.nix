# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "aarch64-darwin";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };
    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
  in {
    devShells.${system}.default = pkgs.mkShell {
      shellHook = ''
        $SHELL
      '';

      packages = [
        toolchain
        pkgs.darwin.apple_sdk.frameworks.Foundation
        pkgs.darwin.apple_sdk.frameworks.SystemConfiguration 

        # We want the unwrapped version, "rust-analyzer" (wrapped) comes with nixpkgs' toolchain
        pkgs.rust-analyzer-unwrapped
      ];

      RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
    };
  };
}
