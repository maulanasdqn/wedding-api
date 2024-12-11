{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    crate2nix
    clippy
    surrealdb
    surrealist
    surrealdb-migrations

    (writeScriptBin "helpme" ''
      __usage="
      👋 Welcome to WeddingAPI development environment. 🚀
      If you see this message, it means your are inside the Nix shell ❄️.

      [Info]===============================================================>

      Rustc Version: v${rustc.version}
      Rustup Version: v${rustup.version}
      Cargo Version: v${cargo.version}

      Command available:
        - start:            start project in production ( need to run build first  ) 🛹
        - build:            build project for production
        - dev:              start development server
        - apply-env:        apply environment variable to current shell
        - helpme:           show this messages

      Repository:
        - https://github.com/maulanasdqn/wedding-api
      [Info]===============================================================>
      "
      echo "$__usage"
    '')

    (writeScriptBin "dev" ''
      cargo watch -x "run -p api"
    '')

    (writeScriptBin "start" ''
      echo "Starting project in production mode..."
      ./result/bin/api
    '')

    (writeScriptBin "build" ''
      nix build -f Cargo.nix
    '')

    (writeScriptBin "apply-env" ''
      export $(cat .env | xargs)
    '')
  ];

  shellHook = ''
    helpme
  '';
}
