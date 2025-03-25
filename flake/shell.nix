{ mkShell, rustc, cargo, rustfmt, clippy, pkg-config, openssl, rust-analyzer }:
mkShell {
  name = "grainfetch";
  packages = [ rustc rustfmt cargo clippy pkg-config openssl rust-analyzer ];
}
