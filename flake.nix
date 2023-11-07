{
    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    inputs.flake-utils.url = "github:numtide/flake-utils";
    inputs.rust-overlay.url = "github:oxalica/rust-overlay";
    inputs.crane.url = "github:ipetkov/crane";
    inputs.crane.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        craneLib = (crane.mkLib pkgs).overrideToolchain pkgs.rust-bin.stable.latest.default;

        # Rust source
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        # Common arguments for building/testing
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = with pkgs; [
            # Additional buildInputs for Linux
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional buildInputs for macOS
            libiconv
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];
        };

        # Build *just* the crate dependencies so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        discordBot = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          # Don't run tests here, we'll run them in checks
          doCheck = false;
          meta = {
            description = "Discord bot for the Boulder Rust meetup Discord server";
            homepage = "https://github.com/boulderrust/discord_bot";
            license = with pkgs.lib.licenses; [mit asl20];
            maintainers = [];
          };
        });
      in
      {
        checks = {
          inherit discordBot;

          # Run clippy, denying warnings
          run-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          # Check formatting
          run-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Run tests with cargo-nextest
          run-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        };
        packages.default = discordBot;
        packages.container = pkgs.dockerTools.buildLayeredImage {
            name = "discord-bot";
            config = {
              Cmd = [ "${discordBot}/bin/discord_bot" ];
            };
        };
        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = with pkgs; [
            skopeo
            cachix
          ];
        };
      });
}
