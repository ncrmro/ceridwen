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
- **SDA:** GPIO4 (I2C Data)
- **SCL:** GPIO5 (I2C Clock)
- **Button 1 (Left/Prev):** GPIO0 (Input, Pull-up)
- **Button 2 (Right/Next):** GPIO1 (Input, Pull-up)
- **Action (Select/Show):** Press Both Buttons
- **VCC:** 3.3V (to display)
- **GND:** GND (to display and button)

**Board revision:** verify the actual Teyleten board's pin labels. The existing
firmware drives GPIO8 low for its LED; LED polarity/behavior requires a board
check. Keep BOOT/RESET accessible. No battery wiring is specified until the
charging/regulation circuit is selected.

### Alternative Connections for ESP32 (non-C3):
  - SDA: GPIO21
  - SCL: GPIO22
  - VCC: 3.3V
  - GND: GND

## Development, build and simulation

The supported environment is **devenv v2** at the repository root. No global
espup installation or `export-esp.sh` is required for the RISC-V ESP32-C3.

```sh
make setup
make check
make simulate ACTIONS=rrrbrrrr OUTPUT=screen.svg
make build-esp32
make upload-esp32  # only with the intended device connected
```

`make build-esp32` installs a pinned nightly plus rust-src under the project's
`.devenv/state/rustup`; ESP-IDF builds the RISC-V standard library and firmware.
The host tests use devenv's stable Rust. Host tests do not compile or validate
the ESP-IDF hardware binary. See [the iteration guide](../docs/development.md)
for verification status and required physical checks.

The host simulator uses the **same session controller and renderer** as the
firmware. `l`, `r`, and `b` mean left, right, and both. Each command starts from
the first lesson, applies the supplied sequence, and writes a 128×64 SVG.
Wrong answers retry the same lesson; correct answers advance on both buttons.
Six-choice dice lessons page three dice at a time. Arithmetic selects a numeric
answer using left/right and checks it with both buttons.

The existing two-button chord detection uses a 50 ms debounce window; physical
button timing and child usability still need testing.

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
