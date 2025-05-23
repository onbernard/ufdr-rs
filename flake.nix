{
  description = "UFDR-rs flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {self, ...}:
    with inputs;
      flake-utils.lib.eachDefaultSystem (system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            (final: prev: {
              rustToolchain = final.rust-bin.stable.latest.default.override {extensions = ["rust-src"];};
            })
          ];
        };
      in {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # (pkgs.rustPlatform.buildRustPackage {
            #   pname = ""; # Copy from cargo.toml
            #   version = ""; # Copy from cargo.toml
            #   src = pkgs.fetchFromGitHub {
            #     owner = "";
            #     repo = "";
            #     rev = ""; # branch
            #     hash = "";
            #   };
            #   cargoHash = "";
            # })
            rustToolchain
            cargo-bloat
            cargo-edit
            cargo-outdated
            cargo-udeps
            cargo-watch
            rust-analyzer
            dioxus-cli
          ];
          env = {
            RUST_BACKTRACE = "1";
            RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          };
        };
      });
}
