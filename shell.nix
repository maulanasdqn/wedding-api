{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    crates2nix
    clippy
    surrealdb
    surrealist
    surrealdb-migrations
  ];
}
