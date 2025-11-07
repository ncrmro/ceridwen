# ceridwen-esp32

ESP32 firmware for the Ceridwen educational system with SSD1306 OLED display support.

## Features

- Uses the shared `ceridwen-core` library for lesson logic
- SSD1306 OLED display (128x64) support via I2C
- Display lessons on the OLED screen
- ESP32 hardware abstraction layer (HAL)

## Hardware Requirements

- ESP32 development board
- SSD1306 128x64 OLED display (I2C)
- Connections:
  - SDA: GPIO21
  - SCL: GPIO22
  - VCC: 3.3V
  - GND: GND

## Building

### Quick Start with Make (Recommended)

From the project root:

```bash
# Enter development shell (auto-installs ESP toolchain on NixOS)
make shell

# Build ESP32 firmware
make build-esp32

# Build and flash to device
make upload-esp32

# Clean build artifacts
make clean-esp32
```

### NixOS Setup

The project includes a Nix flake (`flake.nix`) that provides a complete development environment:

**What it provides:**
- ESP Rust toolchain (via espup, installed automatically)
- espflash and ldproxy tools
- Proper libclang configuration for bindgen
- Compatible library versions (libxml2_13, zlib) for NixOS

**First-time setup:**
```bash
make shell  # Automatically runs espup install and configures environment
```

**Configuration details:**
- ESP-IDF v5.2
- esp-idf-hal v0.45+
- esp-idf-svc v0.51+
- esp-idf-sys v0.36+
- Uses Nix's libclang with libxml2_13 for bindgen compatibility on NixOS

### Manual Setup (Without Nix)

1. Install Rust and the ESP32 toolchain:
```bash
cargo install espup
espup install
. $HOME/export-esp.sh
```

2. Install additional tools:
```bash
cargo install ldproxy espflash
```

3. Build and flash:
```bash
cd ceridwen-esp32
cargo build --features esp32
cargo run --features esp32
```

## Running Tests

The library tests can be run without ESP32 hardware:

```bash
cargo test --package ceridwen-esp32
```

These tests verify:
- Lesson formatting for small displays
- Text truncation
- Display coordinate calculations

## Architecture

The ESP32 crate consists of:

- **lib.rs**: Testable display utilities and formatting functions
- **main.rs**: ESP32 firmware with hardware initialization and display logic

## Display Features

The firmware displays:
1. Welcome message on startup
2. Lesson information including:
   - Lesson type (Subitizing, Addition, Subtraction, Multiplication)
   - Question text (truncated to fit the small screen)
   - Answer or dice pattern for subitizing lessons

## Development

The crate uses feature flags to separate testable code from ESP32-specific code:
- Tests run without the `esp32` feature
- Firmware requires the `esp32` feature to enable ESP32 HAL dependencies

This allows for rapid development and testing without needing ESP32 hardware.
