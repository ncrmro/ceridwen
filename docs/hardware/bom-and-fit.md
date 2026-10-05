# Handheld proof of concept: BOM and assembly fit

Updated 2026-10-04. Status: selected component inventory plus a complete parametric mechanical concept; physical fit and the battery power circuit remain unverified.

See the [project changelog](../../CHANGELOG.md) for the decision history and superseded component choices.

The user initially required complete component measurements before enclosure design, then explicitly authorized a finished parametric concept using editable assumptions on 2026-10-04. Independent models and derived mounts now implement that direction. The previously suggested 100 × 70 × 35–40 mm case is withdrawn as a design constraint: the assembly must determine the case dimensions.

## Component inventory

Dimensions below are millimetres. A published body size is not necessarily the installed envelope. Self-contained simplified component models, derived mounts, a browser CAD workbench, routed harness and solid/motion reports are available. Dimensions remain editable assumptions; final physical fit is unverified. See [the iteration guide](../development.md).

| ID | Qty | Component / candidate | Known geometry | Remaining before mechanical release |
| --- | --- | --- | --- | --- |
| BB1 | 1 | ELEGOO 400 Point Solderless Breadboard, from user's 6-pack | 2.54 pitch; 4 rail sections; side clips and adhesive backing. Earlier 82 × 55 × 9 body estimate is unverified. | Measure body, clips, backing thickness, central channel and grid origin; establish retention surfaces. |
| U1 | 1 | User-selected Teyleten Robot ESP32-C3 Supermini, from 3-pack [S1] | Plant Caravan documents the generic C3 SuperMini footprint as 22.5 × 18, with 2.54 header pitch [L4]; not yet verified for this exact board. | Confirm PCB revision and measure populated height, USB shell/overhang, antenna, boot/reset access, solder joints and installed headers. Record pin-row spacing. Do not use XIAO dimensions. |
| H1–H2 | 2 | MCU pin-header strips | Nominal 2.54 pitch; exact header not selected. | Choose exact strips; model pin length, plastic body, insertion depth and resulting board height. Verify row spacing against breadboard. |
| BAT1 | 1 | User-selected EEMB 3.7 V 502030 250 mAh rechargeable LiPo, JST 2.0 mm, from 4-pack | Project notes use approximately 20 × 30 × 5 [L1]. EEMB publishes 20.5 × 31 × 5.3 for the **bare LP502030 cell** [S2]; its protected-pack store listing specifies 20.5 × 32 × 5.3 [S5]. Lead and connector envelope remains unknown. | Record exact pack revision/protection; measure wrapping, protection board, lead exit and connector. Bare-cell dimensions are not a finished-pack envelope. Confirm charge/discharge ratings and protection. |
| J1 | 1 pair | Detachable battery connection / pigtail | User specifies JST 2.0 mm; exact housing and mated dimensions unknown; power-side termination depends on PWR2. | Match actual battery connector, pitch and polarity; model both mated halves, wire exit, bend allowance and disconnect access. Do not assume JST-PH from appearance. |
| DISP1 | 1 | User-selected Hosyond 0.96 inch OLED I2C/IIC 12864, SSD1306, white, from 5-pack | User confirms product identity, resolution and interface. No verified mechanical drawing for this exact board. | Measure PCB outline/thickness, hole coordinates/diameters, glass, active viewing area and offset, rear parts, header orientation and mated connector height. Record board revision and printed pin order. Screen diagonal is not the PCB envelope. |
| SW1–SW2 | 2 | User-selected Chanzon 6 × 6 × 5 mm tactile momentary switches, 2-pin SPST, from 150-pack | Nominal 6 × 6 × 5 from product title [S4]; installed envelope unknown. | Verify lead spacing/length, insertion depth, body height datum and actuator travel. Model supports and captive caps; verify breadboard insertion. |
| CAP1–CAP2 | 2 | Captive printed button caps | Prototype dimensions in TypeScript; not fit-accepted. | Parameterize around switch travel, return motion and overtravel stops; transmit press forces through supports rather than loose breadboard contacts. |
| PWR1 | 1 provision | Power-off control | Circuit and part not selected. | Choose physical switch or documented sleep/wake scheme; if a switch, add exact part and actuator travel. Check interaction with USB charging and battery isolation. |
| PWR2 | TBD | Battery charging and regulated supply / USB power-path assembly | Required functions unresolved; no module selected. | Verify U1 schematic/revision; do not assume onboard LiPo charging. Choose charging current compatible with BAT1, regulation across its discharge range, and USB/battery isolation. Model every added board, connector and wire; check USB programming and charging connection access. |
| W1 | 1 harness | OLED power/I2C, two button inputs and grounds, breadboard power jumpers | Lengths depend on placement. | Enumerate each wire, gauge, termination and route after layout; include connector bodies, bend radii, strain relief and lid-opening slack. |
| RF1 | Included in U1 | C3 Supermini onboard antenna region | Exact antenna geometry/keepout unverified for selected revision. | Include antenna in U1 model and reserve required RF clearance if radio is used. No separate XIAO antenna/cable is selected. |
| M1 | 1 set | Tray, switch support, battery divider/retainer, display supports | Prototype dimensions in TypeScript; not fit-accepted. | Model before shell; isolate battery from electrical pins; retain without compressing pouch. |
| F1 | 0 | Separate fasteners | Removed: clips are integral to the lid, tray and caps. | Verify snap insertion, retention and release with the chosen print material. |
| I1 | 1 set | Insulation, adhesive/foam where needed, strain relief | Materials/thicknesses not selected. | Include actual thickness and compression limits in stack-up. |
| USB1 | 1 external | USB-C data/charging cable | Exact cable plug unmeasured. | Model plug body, insertion/removal corridor and access to boot/reset. Not retained inside case. |

MCU selection is now the user's Teyleten ESP32-C3 Supermini, replacing the proposed XIAO ESP32-S3. One board is installed per handheld; the other two are spares. Ceridwen already targets ESP32-C3, so the proposed S3 target migration is no longer needed; actual pin labels and board behavior still require verification.

The former XIAO onboard-charging assumption is removed. The selected C3 board's battery charging and power-path capabilities have not been established, so PWR2 remains an explicit BOM gap. A USB-C connector alone does not establish LiPo charging. Do not connect the raw LiPo to 3V3 or assume it can share the USB/5V rail. Verify the selected power circuit and actual pack limits, including USB-connected operation, before closing the electrical BOM. Plant Caravan's charging carrier is a separate module, not evidence that the bare Supermini has charging. Any reused carrier or new power module must have its own dimensions and model.

OLED selection is now fixed to the user's Hosyond white 5-pack product; the Adafruit 326 fallback and its dimensions have been removed. The product title does not establish mounting geometry. Searches on 2026-10-04 did not establish an exact manufacturer drawing, so generic 0.96-inch SSD1306 dimensions must not be used to release its mount or bezel. One module is installed per handheld; the other four are spares.

The user selected the EEMB 3.7V 502030 250 mAh JST 2.0 mm four-pack on 2026-10-04; runtime is not demonstrated. Plant Caravan's sleep-heavy solar estimates do not apply to an illuminated handheld. Measure active/sleep current and USB/battery transitions on the assembled circuit; choose capacity against desired session length.

For the outstanding physical datums and fit-coupon instructions, use the
[measurement sheet](measurements.md). Do not replace missing measurements with
zeros or infer installed dimensions from product names.

## Footprint and enclosure

The current goal constrains all installed electronics to the breadboard X/Y footprint.
The browser/export checker now enforces this independently of collision checks.
See [the enclosure design](enclosure.md) for the proposed stack, wall allowance,
print functions and unresolved mechanical interfaces.

## Required component model record

Each BOM item must carry: stable ID, quantity, exact part/revision, source drawing or dated measurement, dimensional tolerances, units, local origin, physical solid, mounting/pin datums, assembly transform, and separate clearance volumes. Unknown dimensions remain explicitly unknown and block a final-fit claim.

Model populated assemblies, not just bare PCB rectangles. Connector plugs, leads, button motion, battery allowances and cable bends consume space. Vendor STEP is preferred where available; dimensioned simplified solids are acceptable when conservative and marked as such.

Use one TypeScript dimension registry to drive Replicad component models, assembly placement, drawings and fit checks. Artera's `code/packages/cad` supplies the TypeScript/Replicad workflow [L3], but its existing bounding-box checks do not establish detailed handheld interference or assembly access.

## Sequence and acceptance

1. Freeze exact purchased/reused parts and fill every unresolved envelope/interface above. Record manufacturer tolerances and measured dimensions separately.
2. Build component solids and separate keepouts. Align breadboard grid, header pins and actual seating heights; verify electrical row connectivity in the wiring diagram.
3. Arrange components in an open assembly. Current decision: battery above the breadboard on an insulating carrier, keeping internal cable routes inside the board footprint; derive overall case size from the geometry. Show top, side, section and exploded views with IDs matching the BOM.
4. Run broad bounding-box checks, then solid intersection/minimum-distance checks. Report pair IDs and clearance against explicit requirements. Only specifically declared mating contacts are allowed; button sweeps, connector withdrawal corridors and wire volumes also count.
5. Check assembly/removal order, integral clip deflection/release access, lid opening, USB insertion, visible screen area, button motion and battery removal. A collision-free static view alone is insufficient.
6. Generate the shell from the accepted assembly plus wall thickness and manufacturing allowances. Re-run interference checks including shell, fasteners and cap stops. Keep battery clearance based on pack requirements, not an arbitrary tight press fit.
7. Print fit coupons for breadboard retention, USB opening, display mount and buttons; then test the full assembled print. CAD clearance does not establish physical fit or electrical behavior.

Deliverables before calling the case ready: resolved BOM, source-backed component registry, component models, dimensioned assembly/wiring drawings, browser assembly preview, reproducible clearance report, shell STEP/STL and physical fit results. Current status: five printed pieces include integral supports, snap retention and button stops; there are no loose fasteners. Routed wires and case exports are included. Physical acceptance and power-circuit validation remain incomplete. See [the design guide](parametric-design.md).

## Sources

- S1: User-supplied product identity, 2026-10-04: “Teyleten Robot ESP32-C3 Development Board ESP32 Supermini Development Board ESP32 Development Board WiFi Bluetooth 3pcs”. Exact mechanical drawing and power schematic remain unverified.
- [S2: EEMB LP502030 bare-cell product specification](https://www.eemb.com/product-130), checked 2026-10-04. This explicitly describes a bare cell with solder tabs.
- S3: User-supplied product identity, 2026-10-04: “Hosyond 5 Pcs 0.96 Inch OLED I2C IIC Display Module 12864 128x64 Pixel SSD1306 Mini Self-Luminous OLED Screen Board Compatible with Arduino Raspberry Pi (White)”. Mechanical dimensions remain unverified.
- S4: User-supplied Chanzon 150pcs 6x6x5mm tactile momentary 2-pin SPST switch product identity, 2026-10-04. Nominal product-title dimensions only; lead geometry and travel unverified. Supersedes Omron B3F-1000 candidate.
- L1: `/home/ncrmro/repos/ncrmro/plant-caravan/hardware/outdoor/battery-solar-analysis.md`, selected EEMB 502030 and approximate dimensions; inspected 2026-10-04. Does not establish current stock or exact protected-pack variant.
- L2: `/home/ncrmro/repos/artera/artera/docs/milestones/M1-pod-platform-earth-analogue/alpha-breadboard-build.md`, documents XIAO ESP32-S3 Sense bench hardware; inspected 2026-10-04. No specific small battery identified in the inspected Artera COTS records.
- L3: `/home/ncrmro/repos/artera/artera/code/packages/cad/README.md`, Replicad/OpenCASCADE, STEP generation and browser GLB preview; inspected 2026-10-04.
- L4: `/home/ncrmro/repos/ncrmro/plant-caravan/hardware/docs/ESP32-C3-SuperMini.md`, generic Supermini dimensions; inspected 2026-10-04. Form-factor reference only, not an exact Teyleten board drawing or electrical authority.

- [S5: EEMB LP502030 protected-pack store listing](https://eemb.store/products/lp502030), checked 2026-10-04. Lists 20.5 × 32 × 5.3 mm and PCM protection; connector polarity must be checked against the actual pack.
