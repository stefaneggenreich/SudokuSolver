let
  # Pinned nixpkgs, deterministic. Last updated: 2/12/21.
  pkgs = import <nixpkgs>{}; 

  # Rolling updates, not deterministic.
  # pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in
pkgs.mkShell {
  buildInputs = [ pkgs.cargo pkgs.rustc ];
}
