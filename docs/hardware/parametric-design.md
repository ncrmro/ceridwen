# Five-piece snap-together handheld

The enclosure now has **five printed pieces and no separate fasteners**. This
supersedes both the screw design and its 24-pin replacement. Component dimensions
remain editable assumptions; the digital design can be finished and iterated
before physical acceptance.

## Part-count reduction

| Item | Previous | Current |
| --- | ---: | ---: |
| Base | 1 | 1 |
| Electronics tray | 1 | 1 |
| Separate display/switch mounting deck | 1 | 0 |
| Lid | 1 | 1 |
| Separate button-stop plates | 2 | 0 |
| Button caps | 2 | 2 |
| Loose push-pins | 24 | 0 |
| **Total printed pieces** | **32** | **5** |

That removes 27 pieces, about 84%. The electronics and ten modeled wire routes
are unchanged. Caps remain separate so button travel does not depend on flexing
the whole lid on every press. The removable tray remains separate so the
breadboard, battery and wiring are accessible. Combining either further would
compromise assembly access or introduce a different button mechanism.

The body is now 88 × 61 × 38.25 mm with default parameters, excluding button
protrusion. Removing the fastening rails saves 6 mm of width. The top has only
the display and button openings; four small release windows are on the sides.

## What snaps together

- Four cantilevers molded into the lid engage side windows in the base. Press
  them inward through the windows to release the lid.
- The electronics tray rests on ledges outside the breadboard outline and is
  captured by the closed lid. It needs no separate latch or fastening step.
- OLED edge clips and seating bosses are part of the lid; PCB holes are unused.
- Switch cradles and button travel stops are part of the lid. There is no deck
  or removable stop plate.
- Each cap has two slotted flexure ears. It inserts through the top opening;
  the ears compress into their relief slots, then recover below the bezel.
- Power-board and connector clips are integral to the electronics tray.
- A loose battery pocket and a lid-mounted bridge limit movement without
  clamping the pouch. Battery clearance must remain positive.

There are 20 named integral retention features, not 20 separate pieces.
`out/snaps.json` records their owner, retained part and intended release access.
No screws, nuts, inserts or loose pins appear in the assembly or exports.

## Assembly and service sequence

1. Inspect and deburr the prints. Check the flexure slots are clear and the
   material tolerates the required snap travel. The design has not yet been
   physically printed or fatigue-tested.
2. Drop the breadboard into the base and install the MCU/USB bench wiring.
   The support ledges sit outside the board outline to preserve insertion access.
3. Route wiring through the electronics tray and lower it onto the base ledges.
   Fit the battery, connector and selected power hardware into the open tray.
   Leave the unvalidated battery circuit unpowered.
4. With the lid upside down, seat the OLED against its bosses and engage its
   edge clips. Snap each switch into its cradle, keeping leads in the open
   central corridor. Connect the screen and switch wiring before closure.
5. Insert the button caps through the top, compressing their integral ears.
   Their ears retain them below the bezel; molded stops limit downward travel.
   Confirm free return and that neither switch is preloaded.
6. Lower the lid evenly until all four side hooks engage. The lid captures the
   tray and places the battery restraint above the pack with clearance. Keep
   wire slack inside the routing area, away from the seam and clips.
7. To service, release the side hooks and lift the lid gently while managing the
   wire slack. Component clips are exposed with the lid removed. Depress cap
   ears from underneath to remove caps upward.

This is an intended assembly sequence, not physical insertion-force acceptance.
In particular, screen/switch clip stiffness, cap snap travel, simultaneous lid
release and repeated opening require a trial print. The seated collision check
is not a simulation of elastic insertion or fatigue.

## Parameters and self-contained models

Each component owns its solids, dimensions and mounting datums under
`hardware/cad/src/models/`. The selected board, OLED, switches, battery and
breadboard are unchanged. `cap.ts` owns the cap flexures. `parameters.ts` collects
these dimensions; `layout.ts` derives positions; `enclosure.ts` builds the five
pieces; `harness.ts` derives the ten wire routes.

Use the browser component selector to edit measurements. The `snap` group
controls cantilever thickness/length, hook engagement, width and clearance.
The `cap` group includes ear projection, arm thickness and relief clearance.
Defaults are engineering assumptions, not printer/material-specific fits.

A thicker battery raises the wiring datum, screen, switches and lid together.
The lower tray stays at the breadboard/wire clearance height. `upperAllowance`
is vertical packaging space, not a separate printed deck.

The browser worker and CLI share `build-design.ts`. A change rebuilds actual
OpenCASCADE solids and the report, not only a visual bounding box. Save/load
parameter JSON (`schema: 2`) to retain a variant. Older screw/pin parameter files
need the current `snap` group and cap flexure fields, and `case.upperAllowance`
replaces `case.deckThickness`; start with current defaults and copy measured
component values. The obsolete `pushFit` and `mountRail` values are unused.

```sh
devenv shell -- bash -c 'cd hardware/cad && npm run export -- --config /path/to/ceridwen-parameters.json'
```

## Inspection and exports

The workbench shows closed, transparent and exploded views. The report checks
solid validity and connectedness, seated component/wire intersections, button
travel and breadboard header alignment. Integral clips must clear retained parts
in the seated state; no fastener overlap exceptions remain. Header insertion
and named electrical terminations remain explicitly identified.

`out/enclosure/` contains exactly five STL parts and their STEP counterparts.
`out/components/` contains independent local component models;
`component-metadata.json` records their assembly transforms and datums.
`assembly-with-case.step` contains the full geometric assembly, and `bom.csv`
lists the physical entities. The display image is a six-dot lesson preview.

STLs sit on Z=0; lid and caps are flipped. Some clip overhangs and flexure relief
may need slicer supports or orientation changes. Avoid trapping support in the
slots. Physical fit, material selection, battery electronics, connector polarity
and manufacturing acceptance remain separate from digital geometry PASS.
