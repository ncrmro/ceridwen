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

### Prerequisites

1. Install Rust and the ESP32 toolchain:
```bash
# Install espup for ESP32 Rust development
cargo install espup
espup install

# Source the environment
. $HOME/export-esp.sh
```

2. Install additional tools:
```bash
cargo install ldproxy
cargo install espflash
```

### Build the firmware

```bash
cd ceridwen-esp32
cargo build --features esp32
```

### Flash to ESP32

```bash
cargo run --features esp32
```

This will build, flash, and monitor the device.

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
