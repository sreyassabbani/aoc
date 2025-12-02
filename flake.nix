{
  description = "AoC Rust workspace dev environment";

  inputs = {
    # You can swap this for "github:NixOS/nixpkgs/nixos-unstable"
    # if you don't want to use FlakeHub.
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0";
  };

  outputs = { self, nixpkgs, ... }:
  let
    systems = [
      "aarch64-darwin"
    ];

    forAllSystems = f:
      nixpkgs.lib.genAttrs systems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        f pkgs
      );
  in {
    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
        ];

        # Optional: give rust-analyzer access to std sources
        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

        shellHook = ''
          echo "[aoc] Rust dev shell ready"
          echo "  rustc:   $(rustc --version)"
          echo "  cargo:   $(cargo --version)"
          echo "  analyzer: rust-analyzer (via Nix)"
        '';
      };
    });
  };
}

