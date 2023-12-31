{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    buildInputs = [
        pkgs.gtk4
        pkgs.pkg-config
    ];
}