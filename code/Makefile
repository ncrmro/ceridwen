# Makefile for building the ceridwen-esp32 project on NixOS

.PHONY: build-esp32 shell clean-esp32 upload-esp32

shell:
	@echo "Entering dev shell — direnv allow at repo root or run 'devenv shell'..."

build-esp32:
	@echo "Building ceridwen-esp32..."
	@bash -c "source ~/export-esp.sh && cd ceridwen-esp32 && cargo build -Zbuild-std=std,panic_abort --features esp32"

upload-esp32: build-esp32
	@echo "Uploading to device..."
	@bash -c "source ~/export-esp.sh && espflash flash --baud 921600 --monitor target/riscv32imc-esp-espidf/debug/ceridwen-esp32"

clean-esp32:
	@cd ceridwen-esp32 && cargo clean
