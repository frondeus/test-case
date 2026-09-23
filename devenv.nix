{ pkgs, ... }:

{
  languages.rust.enable = true;

  packages = [
    pkgs.git
    pkgs.cargo-readme
  ];

  # cargo-get is not packaged in nixpkgs; keep its installation project-local.
  enterShell = ''
    export PATH="$DEVENV_STATE/cargo-tools/bin:$PATH"
    if [ ! -x "$DEVENV_STATE/cargo-tools/bin/cargo-get" ]; then
      cargo install cargo-get --version 1.4.0 --locked --root "$DEVENV_STATE/cargo-tools"
    fi
  '';
}
