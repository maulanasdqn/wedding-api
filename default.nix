{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./src/app/Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
    nativeBuildInputs = [pkgs.openssl pkgs.pkg-config];
    buildInputs = [pkgs.openssl];
  }
