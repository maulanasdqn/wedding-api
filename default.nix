{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./src/app/Cargo.toml).package;
  rustDeps = pkgs.callPackage ./Cargo.nix {inherit pkgs;};
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    cargoDeps = rustDeps;
    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
    nativeBuildInputs = [pkgs.openssl pkgs.pkg-config];
    buildInputs = [pkgs.openssl];
  }
