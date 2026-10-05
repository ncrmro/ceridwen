{ pkgs, config, ... }:
{
  packages = with pkgs; [
    nodejs_24 (lib.hiPrio cargo) (lib.hiPrio rustc) (lib.hiPrio rustfmt) (lib.hiPrio clippy) rustup
    espflash ldproxy llvmPackages.libclang pkg-config gcc cmake ninja python3 git git-lfs
    iproute2
  ];
  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib pkgs.libxml2_13 pkgs.zlib ];
  env.CARGO_HOME = "${config.devenv.root}/.devenv/state/cargo";
  env.npm_config_cache = "${config.devenv.root}/.devenv/state/npm";
  env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH = "${pkgs.chromium}/bin/chromium";
  processes.cad.exec = ''
    cd hardware/cad
    exec node node_modules/vite/bin/vite.js --host 0.0.0.0 --port 4310
  '';
}
