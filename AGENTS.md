# Ceridwen Project Context

## Project Overview

**Ceridwen** is a Rust-based educational system designed to teach subitizing (instant recognition of quantities) and basic arithmetic. The project is structured as a Rust workspace with a shared core library and multiple frontends (Terminal UI and ESP32 Firmware).

### Architecture

The project is organized into a Cargo workspace with three main members:

1.  **`ceridwen-core`**: The heart of the system.
    *   Contains business logic, data structures, and the lesson database.
    *   **Key Types:** `Lesson` (struct), `LessonType` (enum: Subitizing, Addition, Subtraction, Multiplication), `LessonManager` (in-memory database), `LessonQuery`.
    *   **Design:** `no_std` compatible (implied by usage in ESP32) to support embedded targets.

2.  **`ceridwen-tui`**: A Terminal User Interface application.
    *   Built with `ratatui` and `crossterm`.
    *   Provides a rich interface for browsing lessons and interactive exercises.
    *   Features keyboard navigation and emoji-based visualizations.

3.  **`ceridwen-esp32`**: Firmware for ESP32 microcontrollers.
    *   Built on `esp-idf-hal` and `esp-idf-svc`.
**Hardware:**
- ESP32 development board (ESP32-C3 recommended)
- SSD1306 OLED (I2C): SDA=GPIO4, SCL=GPIO5
  - *Note: Moved from GPIO8/9 to avoid conflict with onboard LED on GPIO8.*
- Button 1 (Left/Prev): GPIO0
- Button 2 (Right/Next): GPIO1
- Action (Select/Show): Press Both Buttons
- Onboard LED: GPIO8 (Software disabled)

See [ceridwen-esp32/README.md](ceridwen-esp32/README.md) for complete build instructions, NixOS setup, and development details.
    *   Displays lessons and interacts via simple inputs (currently set up for I2C display output).

## Build & Development Environment

The project relies heavily on **Nix** for managing the development environment, especially for the ESP32 toolchain.

### Prerequisites

*   **Rust Toolchain:** Standard Rust installation for TUI/Core.
*   **Nix:** Required for ESP32 development to provision the `esp-idf` toolchain, `bindgen` dependencies, and other system tools.

### Key Commands

| Task | Command | Context |
| :--- | :--- | :--- |
| **Run TUI** | `cargo run --package ceridwen-tui` | Runs the terminal application on host |
| **Test Core** | `cargo test --package ceridwen-core` | Runs unit tests for the shared logic |
| **Build ESP32** | `make build-esp32` | Uses `nix develop` to build firmware |
| **Upload ESP32**| `make upload-esp32` | Flashes firmware to connected device |
| **Clean ESP32** | `make clean-esp32` | Cleans ESP32 build artifacts |

### Nix & ESP32 Workflow

The `Makefile` wraps `nix develop` commands. When working with the ESP32 crate manually (outside the Makefile), ensure you are in the nix shell and have sourced the environment:

```bash
nix develop
source ~/export-esp.sh  # Sourced automatically by Makefile targets
```

## Design Philosophy

*   **Target Audience:** Children (Pre-K to Early Elementary).
*   **UI Principles:**
    *   **Minimalist:** Remove technical jargon (e.g., "Lesson 1/24"). Focus on the immediate task.
    *   **Visual:** Prefer graphics (dice, icons) over text where possible.
    *   **Forgiving:** Navigation should be simple and robust.
    *   **Clear Feedback:** "Correct!" or "Try Again" should be immediate and obvious.

## Codebase Conventions

*   **Error Handling:**
    *   `anyhow` is used in `ceridwen-esp32` for top-level error management.
    *   `std::io::Result` is used in `ceridwen-tui`.
*   **Logging:**
    *   ESP32 uses `esp_idf_svc::log` and the standard `log` crate.
*   **Embedded Graphics:**
    *   `ceridwen-esp32` uses `embedded-graphics` and `ssd1306` crates for display rendering.
*   **Lesson Data:**
    *   Lessons are currently hardcoded in `LessonManager::with_defaults()` in `ceridwen-core`.

## Key Files & Directories

*   `ceridwen-core/src/lessons.rs`: Defines `Lesson` struct and `LessonManager` logic. This is where new lesson types or content would be added.
*   `ceridwen-tui/src/main.rs`: Entry point for the TUI, handling the main event loop and terminal setup.
*   `ceridwen-tui/src/ui.rs`: (Inferred) TUI rendering logic.
*   `ceridwen-esp32/src/main.rs`: Entry point for ESP32 firmware, handling hardware init (I2C, Display) and main loop.
*   `flake.nix`: Defines the system dependencies and shell environment for ESP32 development.
*   `Makefile`: Convenience wrappers for Nix-based build commands.
