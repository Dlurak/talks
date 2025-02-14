{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in {
        defaultPackage = pkgs.stdenv.mkDerivation {
          name = "osmapp-lighning-talk";
          rev = "1.0.0";
          src = ./src;
          buildPhase = ''
            ${pkgs.marp-cli}/bin/marp main.md -o main.html
          '';
          installPhase = ''
            mkdir -p $out/share
            cp main.html $out/share
          '';
        };
        devShell = with pkgs;
          mkShell {
            buildInputs = [marp-cli marksman];
          };
      }
    );
}
