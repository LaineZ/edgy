{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, utils, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        toolchain = with fenix.packages.${system};
          combine (with complete; [
            rustc
            cargo
            clippy
            rustfmt
            rust-analyzer
          ]);
      in
      {
        devShell = with pkgs; mkShell rec {
          buildInputs = [
            toolchain
            SDL2
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          RUST_LOG = "warn";
        };
      }
    );
}

