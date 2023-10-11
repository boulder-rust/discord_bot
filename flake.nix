{
    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    inputs.flake-utils.url = "github:numtide/flake-utils";
    inputs.rust-overlay.url = "github:oxalica/rust-overlay";


  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        version = pkgs.lib.attrByPath ["package" "version"] "0.0" (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "discord_bot";
          version = version;

          src = builtins.path {path = ./.; name = "discord_bot_src"; };
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            rustc
            cargo
          ];
          
          buildInputs = with pkgs; [
            # None yet for Linux
          ] ++ lib.optional hostPlatform.isDarwin [
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          meta = {
            description = "Discord bot for the Boulder Rust meetup Discord server";
            homepage = "https://github.com/boulderrust/discord_bot";
            license = with pkgs.lib.licenses; [mit asl20];
            maintainers = [];
          };
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [
            self.packages.${system}.default
          ];
        };
      });
}
