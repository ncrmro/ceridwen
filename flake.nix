{
  description = "A development flake for Ceridwen ESP32";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # Don't install Rust here - use espup's ESP Rust toolchain
            espflash
            espup
            ldproxy
            # Required for bindgen
            llvmPackages.libclang
            # Required for the xtensa toolchain
            gcc
            # Required for ESP toolchain's libclang (provides libstdc++.so.6)
            stdenv.cc.cc.lib
            # Required for esp-clang's libclang (using older version for compatibility)
            libxml2_13
            zlib
          ];

          shellHook = ''
            echo "ESP32 Development Environment"
            echo "================================"

            # Check if ESP toolchain is already installed
            if [ ! -f "$HOME/export-esp.sh" ]; then
              echo "Installing ESP Rust toolchain (first time setup)..."
              espup install
            else
              echo "ESP Rust toolchain already installed"
            fi

            # Source the ESP environment first
            if [ -f "$HOME/export-esp.sh" ]; then
              source "$HOME/export-esp.sh"
              echo "ESP environment loaded successfully"
            else
              echo "Warning: Could not find ~/export-esp.sh"
            fi

            # Set up libclang for bindgen AFTER sourcing export-esp.sh
            # (Use Nix's libclang instead of ESP toolchain's to avoid missing library issues)
            export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
            export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.libxml2_13.out}/lib:${pkgs.zlib.out}/lib:''${LD_LIBRARY_PATH:-}"
            echo ""
          '';
        };
      });
}
