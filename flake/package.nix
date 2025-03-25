{ rustPlatform, pkg-config, openssl }:
let cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in rustPlatform.buildRustPackage (_finalAttrs: {
  pname = cargoToml.package.name;
  inherit (cargoToml.package) version;

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [ openssl ];

  doCheck = false;
})
