{
  pkgs ? import <nixpkgs> {},
  rustPackage ? import ./default.nix {inherit pkgs;},
}:
pkgs.dockerTools.buildImage {
  name = "wedding-api";
  fromImage = pkgs.dockerTools.pullImage {
    imageName = "alpine";
    imageTag = "latest";
    sha256 = "sha256:<correct-alpine-image-digest>"; # Replace with actual sha256
  };
  config = {
    Cmd = ["/app/api"];
    WorkingDir = "/app";
  };
  copyToRoot = {
    "/app" = rustPackage;
  };
  extraCommands = ''
    chmod +x /app/api
  '';
}
