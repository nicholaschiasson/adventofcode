{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust/rust-toolchain.toml;
        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
        ];
        buildInputs = with pkgs; [
          cargo-watch
          delve
          dotnet-sdk
          go
          gopls
          just
          iconv
          lldb
          nil
          nixfmt-rfc-style
          rust-analyzer
          starship
          watchexec
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          inherit buildInputs nativeBuildInputs;
          shellHook = ''
            source .env
            rustc --version
            eval "$(starship init bash)"
          '';
        };
      }
    );
}
