#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# C3 is RISC-V. It needs nightly build-std, not the Xtensa espup toolchain.
toolchain=nightly-2026-04-01
export RUSTUP_HOME="$PWD/.devenv/state/rustup"
rustup toolchain install "$toolchain" --profile minimal --component rust-src
export RUSTC="$(rustup which --toolchain "$toolchain" rustc)"
export RUSTDOC="$(rustup which --toolchain "$toolchain" rustdoc)"
nightly_cargo="$(rustup which --toolchain "$toolchain" cargo)"
build_manifest="$PWD/.devenv/state/firmware-build.jsonl"
cd ceridwen-esp32
"$nightly_cargo" build --locked -Zbuild-std=std,panic_abort --features esp32 --message-format=json > "$build_manifest"
python3 ../scripts/collect-firmware.py "$build_manifest"
