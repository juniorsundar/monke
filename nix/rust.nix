{ fenix, system }:
fenix.packages.${system}.combine [
  fenix.packages.${system}.stable.cargo
  fenix.packages.${system}.stable.rustc
  fenix.packages.${system}.stable.clippy
  fenix.packages.${system}.stable.rust-src
  fenix.packages.${system}.stable.rustfmt
  fenix.packages.${system}.stable.rust-analyzer
]
