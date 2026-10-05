# Ceridwen Project Context

## Project Overview

**Ceridwen** is a Rust-based educational system designed to teach subitizing (instant recognition of quantities) and basic arithmetic. The project is structured as a Rust workspace with a shared core library and multiple frontends (Terminal UI and ESP32 Firmware).

### Architecture

The project is organized into a Cargo workspace with three main members:

1.  **`ceridwen-core`**: The heart of the system.
    *   Contains business logic, data structures, and the lesson database.
    *   **Key Types:** `Lesson` (struct), `LessonType` (enum: Subitizing, Addition, Subtraction, Multiplication), `LessonManager` (in-memory database), `LessonQuery`.
    *   **Design:** Uses `std`, supported by the ESP-IDF target; it is not currently a `no_std` crate.

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

Use **devenv v2** for dependencies and services. The repository retains its root
Cargo workspace; the `code/` layout on `chore/devenv-2` has not been merged.

- `make setup`: locked npm dependencies for CAD through devenv.
- `make check`: host formatting/tests and CAD build/exports.
- `make up`: devenv-managed CAD workbench; actual URL in `hardware/cad/.dev-server.json`.
- `make simulate ACTIONS=rrrbrrrr OUTPUT=screen.svg`: real firmware renderer on host.
- `make build-esp32`: pinned project-local nightly, ESP-IDF C3 build, no flashing.
- `make upload-esp32`: build and flash connected device.
- `make package`: provisional iteration archive.

See [docs/development.md](docs/development.md), the [hardware BOM](docs/hardware/bom-and-fit.md),
and [CHANGELOG.md](CHANGELOG.md). Do not infer final fit from provisional box
models. Resolve all component dimensions and assembly clearances before enclosure
release. Use selected Teyleten C3, Hosyond OLED, Chanzon 2-pin buttons, EEMB JST
2.0 mm battery, and ELEGOO 400-point breadboard; do not substitute earlier candidates.

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
*   `devenv.nix`: Defines system dependencies and CAD server for devenv v2.
*   `Makefile`: Convenience wrappers for Nix-based build commands.

## Parametric mechanical design

The user has explicitly authorized finishing the digital assembly with editable
assumed dimensions; do not block CAD iteration on missing physical measurements.
Keep the distinction between a complete mechanical concept and manufacturing or
battery-circuit acceptance. Components own parameters, solids and mounting datums
in `hardware/cad/src/models/`. `parameters.ts`, `layout.ts`, `enclosure.ts` and
`harness.ts` generate the assembly; the browser worker and CLI share `build-design.ts`.
See `docs/hardware/parametric-design.md`. Parameter changes must rebuild mounts,
ports, case, hardware and wire routing, with collision and motion checks.

Use integral snap features and minimize total printed part count (user correction,
2026-10-04). Replacing screws with loose push-pins did not meet the intent.
The current concept has five printed pieces: base, captured electronics tray,
snap lid, and two snap-in button caps. There are no separate fasteners or stop
plates. Keep clips integral, preserve an assembly/removal path, and never squeeze
the battery pouch. Snap force and fatigue still need physical validation.
