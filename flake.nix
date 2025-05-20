{
  description = "UFDR-rs flake";

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
            export GIT_AUTHOR_NAME="Onésime BERNARD"
            export GIT_AUTHOR_EMAIL="bernard.onesime@gmail.com"
            export GIT_COMMITTER_NAME="Onésime BERNARD"
            export GIT_COMMITTER_EMAIL="bernard.onesime@gmail.com"
            uv sync && source .venv/bin/activate
          '';
        };
      });
}
