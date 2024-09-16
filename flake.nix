{
  description = "Convert latex equations to SVG images";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils = {
      inputs.nixpkgs.follows = "nixpkgs";
      url = "github:numtide/flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      rec {
        defaultPackage = packages.mathimg;
        packages.mathimg = pkgs.buildRustPackage {
          pname = "mathimg";
          src = ./.;
          cargoSha256 = "";
        };
      }
    );
}
