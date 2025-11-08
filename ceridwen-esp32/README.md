# ceridwen-esp32

ESP32 firmware for the Ceridwen educational system with SSD1306 OLED display support.

## Features

- Uses the shared `ceridwen-core` library for lesson logic
- SSD1306 OLED display (128x64) support via I2C
- Display lessons on the OLED screen
- ESP32 hardware abstraction layer (HAL)

## Hardware Requirements

- ESP32-C3 development board (or ESP32)
- SSD1306 128x64 OLED display (I2C)

### Current Breadboard Test Setup

**ESP32-C3 Pin Configuration:**
- **SDA:** GPIO6 (I2C Data)
- **SCL:** GPIO7 (I2C Clock)
- **Button:** GPIO0 (other pin to GND)
- **VCC:** 3.3V (to display)
- **GND:** GND (to display and button)

**Note:** GPIO8 and GPIO9 are alternative I2C pins if needed.

### Alternative Connections for ESP32 (non-C3):
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

## Uploading to ESP32

### Prerequisites

1. **Connect your ESP32** via USB cable to your computer
2. **Verify the device is detected**:
   ```bash
   ls /dev/ttyUSB* /dev/ttyACM*
   # Should show something like /dev/ttyUSB0 or /dev/ttyACM0
   ```

3. **Set proper permissions** (if needed):
   ```bash
   # Add your user to the dialout group (one-time setup)
   sudo usermod -a -G dialout $USER
   # Log out and back in for changes to take effect
   
   # Or use sudo with espflash (not recommended for regular use)
   ```

### Upload Methods

#### Method 1: Using Make (Simplest)

From the project root:
```bash
make upload-esp32
```

This will:
- Build the firmware with optimized settings
- Automatically detect your ESP32 device
- Flash the firmware to the device
- Open a serial monitor to view output

#### Method 2: Using espflash Directly

From the project root:
```bash
# In Nix shell
source ~/export-esp.sh
espflash flash --monitor target/riscv32imc-esp-espidf/debug/ceridwen-esp32

# Specify device manually if auto-detection fails
espflash flash --monitor --port /dev/ttyUSB0 target/riscv32imc-esp-espidf/debug/ceridwen-esp32
```

> **Note:** The target path will be `target/xtensa-esp32-espidf/debug/ceridwen-esp32` for original ESP32, or `target/riscv32imc-esp-espidf/debug/ceridwen-esp32` for ESP32-C3. Check your `.cargo/config.toml` for the configured target.

#### Method 3: Build and Flash Separately

```bash
# Build only
make build-esp32

# Flash manually (from project root)
source ~/export-esp.sh
espflash flash --monitor target/riscv32imc-esp-espidf/debug/ceridwen-esp32
# Or for ESP32: target/xtensa-esp32-espidf/debug/ceridwen-esp32
```

### Monitoring Serial Output

After flashing, the monitor will automatically start. You can also run:

```bash
# Using espflash
espflash monitor

# Or specify port
espflash monitor --port /dev/ttyUSB0
```

To exit the monitor, press `Ctrl+C`.

### Troubleshooting Upload Issues

**Device not found:**
- Ensure USB cable is properly connected
- Try a different USB cable (some are charge-only)
- Check `dmesg | tail` for connection messages

**Permission denied:**
- Run `sudo chmod 666 /dev/ttyUSB0` (temporary fix)
- Or add yourself to dialout group (permanent fix, requires re-login)

**Flash fails:**
- Hold the BOOT button on ESP32 while flashing
- Press EN (reset) button after flash completes
- Try a different USB port

**Build errors:**
- Ensure you're in the Nix shell: `nix develop`
- Source ESP environment: `source ~/export-esp.sh`
- Clean and rebuild: `make clean-esp32 && make build-esp32`

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
