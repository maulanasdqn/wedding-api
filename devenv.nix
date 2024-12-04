{pkgs, ...}: {
  packages = [pkgs.bun pkgs.nodejs_22];
  dotenv.enable = true;
}
