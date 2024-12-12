{pkgs, ...}: let
  baseImage = pkgs.ociTools.pullImage {
    imageName = "alpine"; # or another lightweight image
    tag = "latest";
  };
in
  pkgs.dockerTools.buildImage {
    name = "wedding-api";

    # Use the pulled base image
    fromImage = baseImage;

    copyToRoot = pkgs.buildEnv {
      name = "app-env";
      paths = [
        (pkgs.callPackage ./default.nix {})
      ];
    };

    config = {
      Cmd = ["/bin/api"];
      WorkingDir = "/bin";
    };
  }
