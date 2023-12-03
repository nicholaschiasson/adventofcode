{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    legacy.url = "path:./legacy";
  };

  outputs = { self, nixpkgs, flake-utils, legacy }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      legacyShell = legacy.devShells.${system};
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          pkgs.delve
          pkgs.go
          pkgs.gopls
          pkgs.iconv
          pkgs.lldb
          pkgs.nerdfonts
          pkgs.rustup
          pkgs.starship
        ];
        shellHook = ''
          rustup component add clippy rust-analyzer
          eval "$(starship init bash)"
        '';
      };
      devShells.legacy = legacyShell.default;
    });
}
