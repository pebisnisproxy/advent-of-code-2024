{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  # https://devenv.sh/basics/
  env.GREET = "advent of code 2024 <sena@lichtlabs.org>";

  # https://devenv.sh/packages/
  packages = [pkgs.git pkgs.opencommit pkgs.cargo-watch pkgs.nixd pkgs.nil pkgs.alejandra];

  # https://devenv.sh/languages/
  languages.nix.enable = true;
  languages.rust.enable = true;
  languages.rust.channel = "stable";

  # https://devenv.sh/processes/
  processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  enterShell = ''
    hello
    git --version
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/git-hooks/
  git-hooks.hooks.alejandra.enable = true;
  git-hooks.hooks.rustfmt.enable = true;
  git-hooks.hooks.clippy.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
