# Changelog

Project changes and design decisions. The initial entries below were imported
from the 2026-10-04 project discussion; they are not a reconstructed release
history. User-confirmed choices, proposals, and completed work are distinguished
explicitly. Hardware details and evidence live in the
[BOM and fit requirements](docs/hardware/bom-and-fit.md).

## Unreleased

### 2026-10-04 — Return to main and review

- Switched the clean checkout from `chore/devenv-2` to `main` at `30d23e7`.
  The three devenv/workspace-layout commits remain on `chore/devenv-2`;
  they were not merged or discarded. The checkout matched the locally recorded
  `origin/main`; remote updates were not fetched.
- Reviewed the shared Rust core, TUI, and ESP32 firmware. Recorded stale core
  tests, TUI navigation through unused dice slots, incomplete arithmetic answer
  checking, and inconsistent hardware documentation. No firmware fixes were made.
- Test execution was blocked because Cargo was unavailable in the shell.

### 2026-10-04 — Handheld proof-of-concept direction

- **User requirement:** build a battery-powered learning handheld with an ESP32,
  OLED, two buttons, and a small solderless breadboard inside a case. A chunky
  first prototype is acceptable; the interaction reference is a two-button
  Ledger-like device, with a small Game Boy-like handheld form.
- Existing firmware supplies left/right navigation and a both-buttons action.
  Retaining this interaction is the working direction; child usability still
  needs physical testing.
- Located Plant Caravan's half-size breadboard photo and coordinate-based wiring
  diagram in `code/web/src/content/blog/ox-solar-power-spike.mdx`. Reuse the
  documentation approach; its solar/battery circuit is not the handheld layout.
- Located Artera's TypeScript/Replicad/OpenCASCADE workflow in
  `code/packages/cad`. The proposed Ceridwen approach uses one dimension registry
  for component models, placement, and fit checks, with browser previews and
  STEP/STL outputs. Artera currently generates CAD outside the browser and views
  generated GLBs; it does not supply a ready-made browser CAD editor or handheld.
- Proposed a removable breadboard tray, supported button caps, a separated
  battery pocket, USB access, and a screw-fastened back. These are design
  proposals, not validated geometry or fixed placement decisions.

### 2026-10-04 — Breadboard selection

- **User-confirmed:** one ELEGOO 400 Point Solderless Breadboard from the six-pack,
  with 2.54 mm spacing, four rail sections, side clips, and adhesive backing.
- This replaces the initial uncertainty between a 170-point mini board and a
  400-point half-size board. The earlier 82 × 55 × 9 mm estimate remains
  unverified, including clip protrusions and backing thickness.

### 2026-10-04 — Complete the assembly before designing the case

- **User requirement:** identify all parts and their dimensions, model every
  component, and arrange the complete assembly without unintended overlap before
  making the enclosure.
- Created `docs/hardware/bom-and-fit.md`, including connectors, headers, wiring,
  button travel, supports, fasteners, insulation, and service-access clearances.
- Withdrew the earlier suggested 100 × 70 × 35–40 mm case envelope as a design
  constraint. Component placement and required clearances must determine size.
- Defined the sequence: resolve BOM → model components → arrange and route →
  check interference, motion, and assembly access → derive shell → verify print
  fit. A visual preview or bounding-box check alone does not establish full fit.
- Located the **EEMB 502030, 3.7 V, 250 mAh** battery in Plant Caravan's records.
  It remains a reuse candidate, not a confirmed pack selection. Project notes
  estimate 20 × 30 × 5 mm; EEMB lists 20.5 × 31 × 5.3 mm for the bare LP502030
  cell. Neither establishes the complete protected pack, leads, or connector.
- Kept battery runtime unresolved: Plant Caravan's intermittent sensor workload
  cannot establish runtime for an illuminated learning handheld.

### 2026-10-04 — OLED selection

- **User-confirmed:** one Hosyond 0.96-inch white OLED I2C/IIC module, 128×64,
  SSD1306, from the five-pack.
- Removed the proposed Adafruit 326 fallback and its dimensions from the BOM.
  They must not be reused for the Hosyond mount or bezel.
- Exact PCB, mounting holes, glass/viewing-area offsets, populated thickness,
  and installed header/connector geometry remain unverified. The 0.96-inch
  diagonal does not establish the module envelope.

### 2026-10-04 — ESP32 selection and power consequences

- **User-confirmed:** one Teyleten Robot ESP32-C3 Supermini from the three-pack.
  This supersedes the proposed Seeed XIAO ESP32-S3; an S3 firmware target migration
  is no longer part of the plan. Ceridwen already targets ESP32-C3.
- Recorded Plant Caravan's generic Supermini 22.5 × 18 mm footprint as a reference
  only. Exact Teyleten revision, populated height, header spacing, and USB overhang
  still need verification.
- Removed the XIAO-specific battery charging and external-antenna assumptions.
  Model the selected Supermini's antenna region as part of the board.
- Added an explicit unresolved power assembly for battery charging, regulation,
  and USB/battery power handling. The selected C3 board's charging capability has
  not been established; Plant Caravan's separate charging carrier does not prove
  that the bare Supermini includes it. Any added carrier or power module must
  enter the BOM and component model.

### Open at the end of this decision import

- Exact button selection: Omron B3F-1000 is only a proposed candidate.
- Finished battery-pack identity, connector/polarity, charge/discharge limits,
  power circuit, and power-off behavior.
- Measured or drawing-backed installed dimensions for all selected parts,
  header/connector choices, harness routes, supports, and fasteners.
- Component models, assembly layout, clearance report, enclosure CAD, and
  physical fit validation have not been completed. The BOM is not yet ready
  for procurement or mechanical release.

### 2026-10-04 — Confirm buttons and battery pack

- **User-confirmed:** two Chanzon 6 × 6 × 5 mm momentary 2-pin SPST tactile
  switches from the 150-pack. Supersedes the Omron B3F-1000 proposal. Nominal
  product-title dimensions do not establish lead spacing, insertion depth,
  actuator travel, or installed height.
- **User-confirmed:** one EEMB 3.7 V 502030 250 mAh rechargeable LiPo with
  JST 2.0 mm connector from the four-pack. This promotes the battery from a reuse
  candidate to a selected product. Finished pack dimensions, protection details,
  connector housing/polarity, and charge/discharge limits still need verification.
- Charger/regulator and power-off implementation remain unresolved for the C3.

### 2026-10-04 — Establish software and CAD iteration workflow

- **User requirement:** use devenv v2 for dependencies and enable iteration on
  models/software toward a complete designed package.
- Added root `devenv.nix`, `devenv.yaml`, and lockfile; removed legacy flakes.
  Kept the root Cargo layout. The other branch's workspace move remains unmerged.
  Node, Rust, Python, Chromium, and ESP build tools are provided through devenv;
  npm and Cargo libraries have lockfiles and project-local caches.
- Added a TypeScript assembly registry, Three.js browser workbench, JSON
  editing/import/export, exploded view, and envelope-spacing checks. Replicad
  exports 13 component-envelope STEP/STL models and an assembly STEP. These are
  explicitly provisional box proxies, not detailed component or enclosure CAD.
- Added fit regression tests and an isolated browser smoke test exercising an
  introduced battery collision and JSON round trip. Inspected the rendered
  desktop preview. The manufacturing-release check rejects unresolved work
  even when simple envelope spacing passes.
- Repaired stale core tests and arithmetic question formatting; constrained TUI
  dice selection to populated options. Extracted a hardware-independent
  two-button session shared by firmware and the SVG simulator. Wrong answers
  retry the same lesson; arithmetic answers are checked; six-dice exercises page
  three choices at a time to fit the OLED.
- Verified 22 host Rust tests, Clippy with warnings denied, CAD type checks and
  exports, and the browser smoke test. Built the ESP32-C3 binary using
  `nightly-2026-04-01` and ESP-IDF v5.2 through devenv. No device was flashed.
- Added a package command collecting sources, locks, documentation, preview,
  STEP/STL proxies, firmware build outputs, and SHA-256 manifest. It is an
  iteration package, not manufacturing approval. Remote CI has not been run.
- Final completion still requires exact installed dimensions, the power circuit,
  detailed supports/routing/mounts, enclosure geometry, and physical acceptance.

## 2026-10-04 — Breadboard footprint constraint and wiring drawing

- Require every installed component envelope to remain within BB1 in X/Y;
  the external USB insertion corridor may extend outside it. The enclosure
  walls will necessarily add thickness outside the component footprint.
- Add an independent footprint check to the browser and export fit report;
  disjoint parts outside the breadboard now fail even without collisions.
- Publish a label-based USB bench wiring SVG, eight-wire connection list and
  continuity-check assembly sequence. Actual breadboard row mapping, routed
  lengths and the battery supply circuit remain pending.

## 2026-10-04 — Stacked layout and prototype case solids

- Move the battery and power-module reservation above the breadboard onto an
  insulating carrier. The previous under-board arrangement required wire routing
  outside the breadboard footprint and is superseded.
- Center the OLED and align the two switch/cap positions; raise the switches to
  the display level to avoid long unsupported button plungers.
- Add parametric tray, carrier, cover and flanged-cap STEP/STL prototypes, USB
  relief, internal cable passages, screw features and browser case preview.
- Add OpenCASCADE solid-intersection/distance reports and require every printed
  part to be one valid solid. Hide stale case meshes after assembly edits.
- Retain manufacturing blockers: selected power circuit, measured dimensions,
  component supports/retention, cap stops, routed wires, hardware and physical fit.

## 2026-10-04 — Physical interface measurements

- Locate EEMB's protected-pack listing (20.5 × 32 × 5.3 mm), distinct from the
  earlier bare-cell specification. Keep installed lead/connector space provisional.
- Add a component measurement sheet with mounting datums required to finish the
  display, switch, battery and USB interfaces.
- Add STEP/STL coupons for nominal 6 mm switch-body fit and prototype button-guide
  fit, with notch counts identifying three clearance options. These are measurement
  tools, not approval of the unfinished case or button mechanism.

## 2026-10-04 — Complete editable mechanical concept

- User clarified that each component should be self-contained and the finished
  assembled case should be generated now using measurements that can change later.
  Missing physical measurements no longer block the digital design.
- Replace box proxies with independent parametric breadboard, ESP32/header/USB,
  OLED/glass, tactile-switch, battery, connector, power-study, cap and screw models.
- Derive carrier/deck/cover height, mounts, bezel, USB opening, switch saddles,
  travel-stop plates, battery/connector restraints, screw pockets and harness paths.
- Complete eight printed parts, 24 screw instances and ten routed wire branches;
  check solids, the electronic/wire footprint, button swept volumes and header grid.
- Rebuild CAD inside a browser worker after measurement edits. Add parameter
  save/load and current-design STL/STEP download. Export independent local component
  models and placement/anchor metadata for reuse.
- Browser verification changes battery thickness, rebuilds the case, round-trips
  the parameters and exports the changed model. Manufacturing and battery-circuit
  validation remain separate from completion of the digital mechanical concept.

### 2026-10-04 — Push-fit retention replaces screws

- User decision: avoid screws and assume push-to-fit assembly. This supersedes
  the earlier screw-fastened design and screw BOM.
- Replace all 24 screw instances with printable split push-pins, chamfered
  insertion ends and smooth sockets. Retain the eight main printed parts.
- Parameterize shaft/head/slot dimensions and socket interference; preserve loose
  battery restraint. Model intentional socket compression separately from
  unintended collisions, and include pins in solid-validity and button checks.
- Export three printable pin sizes and an instance list; update the browser,
  BOM and assembly instructions. Retention force remains a physical assumption.

### 2026-10-04 — Integral snaps and five-piece enclosure

- User correction: replacing screws with loose push-pins preserved the unwanted
  complexity. Prioritize integral snap assembly and radically reduce part count.
- Reduce 32 printed pieces to five: base, captured electronics tray, snap lid,
  and two snap-in caps. Remove all 24 pins, the upper deck and both stop plates.
- Integrate OLED/switch mounts and button stops into the lid, and power-board/
  connector clips into the tray. Add cap flexure ears for insertion from above.
- Replace fastening rails and top access holes with four side-release lid clips;
  reduce default body width from 94 to 88 mm. Keep the battery loosely restrained.
- Update assembly/service instructions, editable snap dimensions, five-part
  exports and browser part-count display. Snap force/fatigue remain unverified.
