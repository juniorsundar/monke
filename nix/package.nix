{ lib, naersk, stdenv, ... }:
naersk.buildPackage {
  src = ../.;
  # Add any non-Rust build inputs here (e.g., openssl, pkg-config)
  buildInputs = [ ];
  nativeBuildInputs = [ ];

  meta = with lib; {
    description = "My Rust Project";
    platforms = platforms.unix;
  };
}
