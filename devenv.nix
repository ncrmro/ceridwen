{ pkgs, ... }:

{
  packages = with pkgs; [
    rustup
    espflash
    espup
    ldproxy
    llvmPackages.libclang
    gcc
    stdenv.cc.cc.lib
    libxml2_13
    zlib
  ];

  env = {
    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  };

  enterShell = ''
    echo "Ceridwen ESP32 devenv shell"
    echo "================================"

    if [ ! -f "$HOME/export-esp.sh" ]; then
      echo "Installing ESP Rust toolchain (first time setup)..."
      espup install
    else
      echo "ESP Rust toolchain already installed"
    fi

    if [ -f "$HOME/export-esp.sh" ]; then
      # shellcheck disable=SC1090
      source "$HOME/export-esp.sh"
      echo "ESP environment loaded"
    else
      echo "Warning: ~/export-esp.sh not found"
    fi

    # CRITICAL: prepend Nix's stdc++/libxml2/zlib so bindgen finds the
    # right shared libs rather than the older ones bundled with the ESP
    # toolchain.
    export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.libxml2_13.out}/lib:${pkgs.zlib.out}/lib:''${LD_LIBRARY_PATH:-}"
  '';
}
