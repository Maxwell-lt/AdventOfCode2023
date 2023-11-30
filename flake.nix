{
  description = "Advent of Code 2023 Solutions";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
	pkgs = import nixpkgs {
	  inherit system overlays;
	};
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "llvm-tools-preview" ];
        };
	rustPlatform = pkgs.makeRustPlatform {
	  rustc = rust;
	  cargo = rust;
        };
        aocd = pkgs.writeScriptBin "aocd"
          ''
            #! /usr/bin/env bash
            aoc download -d $1 --input-only --input-file input/Day$1
          '';
      in with pkgs; {
        devShell = mkShell {
	  buildInputs = [
	    rust
            diffutils
            rust-analyzer
            cargo-nextest
            grcov
            cargo-llvm-cov
            aoc-cli
            aocd
	  ];

	  shellHook = ''
	    export RUST_BACKTRACE=1
	  '';
	};
  });
}
