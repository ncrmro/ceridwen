# Iterating on Ceridwen

## Environment

Install Nix and **devenv v2**, then run from the repository root:

```sh
make setup
make check
make up
```

`devenv.nix` supplies Node 24, stable Rust/Cargo/Clippy/rustfmt, Python, ESP flashing
and build tools, and the Chromium executable for browser tests. `devenv.lock`
pins Nix inputs; `Cargo.lock` and `hardware/cad/package-lock.json` pin libraries.
The local CLI was verified at devenv 2.2.0; the updated module input is still v2.
The former flake and automatic global espup installation have been removed.
The separate `chore/devenv-2` branch remains untouched; its `code/` move was not
merged into this checkout.

Host Rust binaries take precedence over rustup shims. Cargo's cache lives under
`.devenv/state/cargo`, avoiding global Nix-managed registry conflicts. The C3
cross-build explicitly selects its own pinned nightly compiler and rust-src
under `.devenv/state/rustup`. No secrets or global profiles are needed.

## CAD workbench

`make up` starts the `cad` process with devenv. The preferred port is 4310; Vite
falls forward if occupied and writes the actual URL, PID and working directory to
`hardware/cad/.dev-server.json`. Use that record instead of assuming the port.
Stop the process using `devenv down` when finished.

The browser offers closed, transparent and exploded views with independent wire
toggles. Choose a component and change measurements; a Web Worker builds
OpenCASCADE geometry and rechecks the assembly. Save/load a parameter JSON file to
keep changes. The same generator runs from the CLI:

```sh
make cad
devenv shell -- bash -c 'cd hardware/cad && npm run export -- --config /path/to/ceridwen-parameters.json'
devenv shell -- bash -c 'cd hardware/cad && npm run release:check'
```

Self-contained component modules live under `hardware/cad/src/models/`.
`parameters.ts` collects defaults, `layout.ts` places components, `enclosure.ts`
derives mounts and shell, and `harness.ts` routes wires. Read the
[parametric design guide](hardware/parametric-design.md) for editable datums,
assembly order, assumptions and the complete generated file inventory.

Exports include independent component STEP/STL, print-plane case STLs,
assembled-coordinate case STEP, full assembly STEP, parameter/placement metadata,
BOM, hardware and harness records, and fit reports. `enclosure-report.json` checks
solid intersections, button travel, header-to-grid alignment and footprint.
The manufacturing release command intentionally remains nonzero until physical
fit and battery electronics are validated. It does not prevent concept exports.

Browser verification (start the server first):

```sh
devenv shell -- bash -c 'cd hardware/cad && npx playwright test'
```

The test generates the actual CAD in a browser worker, changes battery thickness,
checks the resulting larger fitted case, saves/reloads parameters, downloads a
current STL and captures closed, transparent, exploded and mobile views. It can
take several minutes because it performs multiple complete CAD builds.

## Software and firmware

```sh
devenv shell -- cargo run -p ceridwen-tui
make simulate ACTIONS=rrrbrrrr OUTPUT=screen.svg
make build-esp32
```

The simulator uses the firmware's Rust session controller and OLED renderer.
`l`, `r`, `b` correspond to left, right and both; each invocation begins at the
first lesson. SVG output has a 128×64 viewBox. Use it to inspect screen changes
without reflashing. `rrrbrrrr` selects the fifth die in the six-choice lesson.

The two-button session checks numeric arithmetic answers, pages dice choices,
retries errors on the same lesson and advances correct answers. The TUI retains
its lesson-browser interface; its dice navigation now respects the valid count.
Host tests cover the controller and all default display states/answer selections.

`make build-esp32` builds for `riscv32imc-esp-espidf` using pinned
`nightly-2026-04-01` and ESP-IDF v5.2. First use downloads Espressif tools and
Python dependencies. All system dependencies come from devenv; generated SDK
state stays in the checkout. `make upload-esp32` explicitly flashes the connected
device. This session does not establish hardware behavior or physical acceptance.

## Package and completion boundary

`make package` runs checks and creates
`hardware/cad/out/ceridwen-iteration-package.zip`, including sources, lockfiles,
docs, browser build, available screenshots, model exports, C3 ELF/bootloader/
partition table from the same build, and a SHA-256 manifest.
This is a **provisional iteration package**, not a finished product release.

Verified locally on 2026-10-04: 22 Rust host tests, host Clippy with warnings
denied, CAD type checks and envelope regression tests, STEP/STL generation,
desktop/mobile browser smoke test with collision feedback and JSON round-trip,
and an ESP32-C3 firmware build. The `release:check` command correctly exits 1
with zero envelope collisions and unresolved design items. No flash, battery
test, printed fit test, or remote CI run has been performed. The USB bench
pin map and power-design boundary are in [wiring.md](hardware/wiring.md).

Before the final design: verify actual component/connector/header dimensions;
select and validate the charger/regulator and power-off circuit; replace assumed component and mounting dimensions with measurements, verify actual
wire terminations and print tolerances, and physically test the assembled enclosure. Consult the [BOM](hardware/bom-and-fit.md)
and [changelog](../CHANGELOG.md). No guessed dimension can establish final fit.
