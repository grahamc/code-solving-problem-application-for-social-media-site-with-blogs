{ pkgs ? import <nixpkgs> {} }:
pkgs.stdenv.mkDerivation {
  name = "logfile-parser";
  version = "0.0.1";
  src = ./default.nix;

  buildInputs = with pkgs; [
    python3
    python35Packages.flake8
    makeWrapper
  ];

  shellHook =''
    find . -name '*.py' | xargs flake8
  '';
}
