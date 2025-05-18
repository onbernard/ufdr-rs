{
  description = "Python project with uv template";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ {self, ...}:
    with inputs;
      flake-utils.lib.eachDefaultSystem (system: let
        pkgs = import nixpkgs {
          inherit system;
          overlay = [];
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [uv maturin];
          shellHook = ''
            uv sync && source .venv/bin/activate
          '';
        };
      });
}
