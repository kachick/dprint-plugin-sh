{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.xz";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      lib = nixpkgs.lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      formatter = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        pkgs.writeShellApplication {
          name = "dprint-fmt";
          runtimeInputs = with pkgs; [
            dprint
          ];
          text = ''
            dprint fmt "$@"
          '';
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        rec {
          dprint-plugin-sh = pkgs.callPackage ./package.nix { };
          default = dprint-plugin-sh;
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.dprint-plugin-sh ];

            buildInputs = with pkgs; [
              bashInteractive
              findutils # xargs
              diffutils # for E2E test
              nixd
              nixf-diagnose
              go-task
              typos
              zizmor

              wasm-tools

              # buildRustPackage does not enable these
              rust-analyzer
              clippy
            ];

            nativeBuildInputs = with pkgs; [
              rustc.llvmPackages.bintools # rust-lld
            ];

            env = {
              # Needed for avoiding "error: linker `rust-lld` not found".
              # Adding packages like binutils is not enough
              #
              # https://github.com/NixOS/nixpkgs/issues/70238
              CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";

              # Workaround for rust-analyzer error
              RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

              # Set NIX_PATH for nixd inlay hints
              NIX_PATH = "nixpkgs=${nixpkgs.outPath}";
            };
          };
        }
      );
    };
}
