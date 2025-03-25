{ writeShellApplication, fd, nixfmt, deadnix, statix, rustfmt }:
writeShellApplication {
  name = "liner";
  runtimeInputs = [
    fd
    nixfmt
    deadnix
    statix
    rustfmt
  ];
  text = ''
    fd -e nix -X nixfmt {} \; -X deadnix -e {} \;
    fd -e nix -x statix fix {} \;
    fd -e rs -X rustfmt {} \;
  '';
}
