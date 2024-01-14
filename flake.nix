{
  description = "A CLI utility that implements binary search.";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        name = "bs";
      in
      {
        defaultPackage = pkgs.rustPlatform.buildRustPackage rec {
          inherit name;
          src = pkgs.lib.cleanSource ./.;

          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = "--release";
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
          ];
          shellHook = ''
            PS1="\n\[\033[01;32m\]${name}(default) >\[\033[00m\] "
          '';
        };
      }
    );
}
