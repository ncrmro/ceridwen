# How Ceridwen was built

Ceridwen is a small two-button learning handheld built around an ESP32-C3,
OLED, battery and 400-point breadboard. This archive records the design's
progression—including an approach we rejected—so others can inspect and
reproduce the work.

| Milestone | Evidence and downloads | Git tag |
| --- | --- | --- |
| Working firmware/TUI baseline, December 2025 | Original existing source commit; no CAD images | [`history/2025-12-21-firmware-baseline`](https://github.com/ncrmro/ceridwen/tree/history/2025-12-21-firmware-baseline) |
| Devenv experiment, June 2026 | Original unmerged `code/` layout; not the current workspace | [`history/2026-06-06-devenv-experiment`](https://github.com/ncrmro/ceridwen/tree/history/2026-06-06-devenv-experiment) |
| Component-envelope study | [Original visual record](2026-10-04-envelope-study/) | [`design/2026-10-04T222549Z-envelope`](https://github.com/ncrmro/ceridwen/tree/design/2026-10-04T222549Z-envelope) |
| First shell study | [Original views and partial source](2026-10-04-first-shell/); no matching model exports recovered | [`design/2026-10-04T225347Z-first-shell`](https://github.com/ncrmro/ceridwen/tree/design/2026-10-04T225347Z-first-shell) |
| Screw-fastened design | [8 printed parts + 24 screws; images, STEP/STL, parameters and source](2026-10-05-screws/) | [`design/2026-10-05T011141Z-screws`](https://github.com/ncrmro/ceridwen/tree/design/2026-10-05T011141Z-screws) |
| Loose push-pin experiment | [32 printed pieces; images, models and source](2026-10-05-push-pins/) | [`design/2026-10-05T012425Z-push-pins`](https://github.com/ncrmro/ceridwen/tree/design/2026-10-05T012425Z-push-pins) |
| Integral snap design | [5 printed pieces, no loose fasteners; full archive](2026-10-05-integral-snaps/) | [`design/2026-10-05T015554Z-integral-snaps`](https://github.com/ncrmro/ceridwen/tree/design/2026-10-05T015554Z-integral-snaps) |

## What changed

| Screws | Loose push-pins | Integral snaps |
| --- | --- | --- |
| ![Screws](2026-10-05-screws/captures/desktop.png) | ![Push-pins](2026-10-05-push-pins/captures/desktop.png) | ![Integral snaps](2026-10-05-integral-snaps/captures/desktop.png) |
| [Mobile view](2026-10-05-screws/captures/mobile.png) | [Mobile view](2026-10-05-push-pins/captures/mobile.png) | [Mobile view](2026-10-05-integral-snaps/captures/mobile.png) |

Replacing screws with loose pins kept the same assembly complexity. The final
revision instead integrates clips, supports and stops into the lid and tray.
It removes the separate deck, two stop plates and 24 fasteners, and narrows the
body from 94 to 88 mm. See [the design guide](../../docs/hardware/parametric-design.md)
for assembly and service access.

![Current exploded assembly](2026-10-05-integral-snaps/captures/exploded.png)

## What the dates and tags mean

The December/June tags point to original existing commits. The October design
work was uncommitted at the time; its milestone tags were created during later
archival. They are **not backdated commits**.

Timestamped design tags use the UTC time at which a preserved screenshot was
observed in the session. That is not an independently verified shutter time.
The historical PNGs are recovered rendered copies of the original screenshot
outputs and may have been resized by the image tool. Fresh captures under
`captures/` have their actual capture times recorded in `capture.json`.

The screw and push-pin tags have separately reconstructed, build-verified CAD
source at `hardware/cad/`, plus `SNAPSHOT.md` explaining their scope. Unchanged
supporting files and firmware/tooling use the current project checkpoint. Each
archive also contains a self-contained CAD source ZIP and recovery provenance.
The integral-snap tag contains the current implementation and its archive.
Early envelope/shell tags preserve visual evidence only; the later-recovered
partial shell source is available on main.

All three full model revisions passed TypeScript/unit checks, seated-solid
interference checks, button travel and header alignment checks, production
builds and desktop/mobile browser capture. These checks establish a reproducible
digital concept, not physical fit or battery-circuit acceptance.

## Download and reproduce

PNG, STEP, STL and ZIP assets are stored in Git LFS; metadata and source remain
normal Git text. Install Git LFS before cloning, or run:

```sh
git lfs install
git lfs pull
python scripts/verify-snapshots.py
```

For a historical full revision, create a separate worktree at its tag. Then use
`make setup` and `make up` through devenv v2. The actual workbench URL is recorded
in `hardware/cad/.dev-server.json`; another running checkout may occupy port 4310.
Source bundles can also be extracted independently and built using the repository's
devenv environment with `npm ci` followed by `npm run build`.

Each full archive contains desktop, mobile and exploded views, a full assembly
STEP, independent component models, printable enclosure parts, parameters,
BOM, routed harness, fit reports and SHA-256 manifests. The mobile captures use
the same default dimensions as the archived model; they are not screenshots of
the temporary thicker-battery test variant.

## Record the next milestone

```sh
devenv shell -- bash -c 'cd hardware/cad && npm run build && node --import tsx scripts/capture.ts'
devenv shell -- python scripts/archive-design.py YYYY-MM-DD-name --provenance 'Describe the source revision and what changed.'
python scripts/verify-snapshots.py
git add hardware/snapshots/YYYY-MM-DD-name
git commit -m 'docs(history): archive name milestone'
git tag -a design/YYYY-MM-DDTHHMMSSZ-name -m 'Describe the capture and revision.'
```

Use the capture's UTC timestamp, update this index, and push the commit and tag.
The archive command refuses to overwrite an existing milestone.
