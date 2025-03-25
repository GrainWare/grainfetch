{
  description =
    "The fetch for grains, written in rust. Now featuring 100% more grains.";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs = { self, nixpkgs, systems, ... }:
    let
      inherit (nixpkgs) lib;
      pkgsFor = nixpkgs.legacyPackages;
      eachSystem = f:
        lib.genAttrs (import systems) (system: f system pkgsFor.${system});
    in {
      packages = eachSystem (system: pkgs: {
        default = pkgs.callPackage ./flake/package.nix { };
        grainfetch = self.packages.${system}.default;
      });

      devShells = eachSystem
        (_: pkgs: { default = pkgs.callPackage ./flake/shell.nix { }; });

      formatter =
        eachSystem (_: pkgs: pkgs.callPackage ./flake/formatter.nix { });
    };
}
