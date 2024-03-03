{
  description = "Sort data by date. For example, if text input has a column given by ISO 8601 date, this tool can sort the data by the date column.";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            just
            cargo-llvm-cov
            # llvm-tools-preview needed for llvm-cov test coverage
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "llvm-tools-preview" ];
            })
            rust-analyzer
          ];
          shellHook = ''
            rustc --version
          '';
        };
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      }
    );
}
