{
  description = "Workspace utility and manager for sway";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-flake.url = "github:juspay/rust-flake";
    rust-flake.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.flake-parts.flakeModules.easyOverlay
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];

      systems = [ "aarch64-linux" "x86_64-linux" ];

      perSystem = { config, self', inputs', pkgs, system, ... }: {
        rust-project = {
          crates.sworker.crane.args = {
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          };
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = config.rust-project.crane-lib.filterCargoSources;
          };
        };

        overlayAttrs = rec {
          inherit (self'.packages) sworker;
          default = sworker;
        };

        packages.default = self'.packages.sworker;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self'.devShells.rust ];
          RUST_BACKTRACE = "full";
        };
      };
    };
}