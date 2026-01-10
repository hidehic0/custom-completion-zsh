{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        toolchain = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
        meta = {
          description = "";
          homepage = "https://github.com/hidehic0/custom-completion-zsh";
          license = pkgs.lib.licenses.mit;
        };
      in
      {
        packages.default = rustPlatform.buildRustPackage {
          name = "custom-completion-zsh";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          inherit meta;
        };
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust-bin.beta.latest.default
          ];
        };
      }
    );
}
