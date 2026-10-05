# All toolchains enter through devenv v2. Run from the repository root.
.PHONY: shell setup check cad up simulate build-esp32 upload-esp32 package
shell:
	devenv shell
setup:
	devenv shell -- npm --prefix hardware/cad ci --no-audit --no-fund
check:
	devenv shell -- cargo fmt --all -- --check
	devenv shell -- cargo test --workspace --locked
	devenv shell -- cargo clippy --workspace --all-targets --locked -- -D warnings
	devenv shell -- npm --prefix hardware/cad run build
cad:
	devenv shell -- npm --prefix hardware/cad run build
up:
	devenv up cad
simulate:
	devenv shell -- cargo run --locked -p ceridwen-esp32 --example simulator -- "$(ACTIONS)" "$(or $(OUTPUT),screen.svg)"
build-esp32:
	devenv shell -- bash scripts/build-esp32.sh
upload-esp32: build-esp32
	devenv shell -- espflash flash --baud 921600 --monitor target/riscv32imc-esp-espidf/debug/ceridwen-esp32
package: check build-esp32
	devenv shell -- python3 scripts/package.py
