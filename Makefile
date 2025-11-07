# Makefile for building the ceridwen-esp32 project on NixOS

.PHONY: build-esp32 shell clean-esp32 upload-esp32

shell:
	@echo "Entering Nix develop shell..."
	@nix develop

build-esp32:
	@echo "Building ceridwen-esp32..."
	@nix develop --command bash -c "source ~/export-esp.sh && cd ceridwen-esp32 && cargo build -Zbuild-std=std,panic_abort --features esp32"

upload-esp32: build-esp32
	@echo "Uploading to device..."
	@nix develop --command bash -c "source ~/export-esp.sh && cd ceridwen-esp32 && espflash flash --monitor"

clean-esp32:
	@cd ceridwen-esp32 && cargo clean
