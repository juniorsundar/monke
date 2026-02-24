{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # naersk = {
    #   url = "github:nix-community/naersk";
    #   inputs.nixpkgs.follows = "nixpkgs";
    # };
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      # naersk,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        # Pull in our custom logic from the nix/ folder
        rustToolchain = import ./nix/rust.nix { inherit fenix system; };
        # naersk' = pkgs.callPackage naersk {
        #   cargo = rustToolchain;
        #   rustc = rustToolchain;
        # };
      in
      {
        # packages.default = pkgs.callPackage ./nix/package.nix {
        #   naersk = naersk';
        # };

        devShells.default = import ./nix/shell.nix {
          inherit pkgs rustToolchain;
        };
      }
    )
    // {
      nixConfig = {
        extra-substituters = [ "https://nix-community.cachix.org" ];
        extra-trusted-public-keys = [
          "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
        ];
      };
    };
}
