{ source ? import ./nix/sources.nix,
  nixpkgs-mozilla ? import source.nixpkgs-mozilla,
  rust-src-overlay ? import "${source.nixpkgs-mozilla}/rust-src-overlay.nix",
  pkgs ? import source.nixpkgs { overlays = [ nixpkgs-mozilla rust-src-overlay ]; },
  nodeModules ? import ./nix/node/global_composition.nix { inherit pkgs; }
}:
let
  rust' = (pkgs.rustChannelOf { channel = "nightly"; date = "2020-12-05";}).rust.override {
    targets = [];
    # targets = ["wasm32-unknown-unknown"];
    extensions = [
      "rust-src"
      "rust-analyzer-preview"
      "clippy-preview"
      "rust-analysis"
      "rust-std"
      # "rustfmt-preview" 
    ];
  };
in pkgs.mkShell {
  buildInputs = [
    rust'
    # pkgs.cmake
    # pkgs.wasm-pack
    # pkgs.wabt
    pkgs.rustracer
    pkgs.latest.rustChannels.stable.rustfmt-preview
    # pkgs.openssl
    # pkgs.python3
    pkgs.llvmPackages.clang-unwrapped
    # pkgs.nodejs
    # pkgs.wasm-pack
    # nodeModules.webpack
    # nodeModules.webpack-cli
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.IOKit
    pkgs.darwin.apple_sdk.frameworks.Security
    pkgs.darwin.apple_sdk.frameworks.CoreServices
  ];
}
