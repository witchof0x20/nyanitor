{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    RUST_LOG="nyanitor=debug";
    buildInputs = with pkgs; [postgresql];
}
