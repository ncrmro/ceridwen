# Component measurements and fit coupons

Status: awaiting measurements of the selected hardware. Record millimetres,
measurement date, instrument resolution and a part/revision photo reference.
Do not compress the battery pouch with calipers. Keep it disconnected while
checking dimensions or mechanical fit.

## Minimum information needed to finish the mounts

| Part | Record | Current evidence / why it matters |
| --- | --- | --- |
| BB1 ELEGOO | Overall X/Y including side clips; height including adhesive; clip protrusions; first grid-hole offset from an edge; centre channel width | 82 × 55 × 9 is still a planning estimate. Determines tray and header alignment. |
| U1 Teyleten | PCB X/Y; header row separation; pin pitch and first-pin offset; PCB thickness; underside-to-breadboard seating height; highest part; USB shell position and actual plug envelope | Generic Supermini dimensions cannot fix this board revision's mounting datums. |
| DISP1 Hosyond | PCB X/Y/thickness; glass X/Y and edge offsets; active viewport X/Y and offsets; mounting hole centres/diameters; rear component height; header position/orientation and mated height | Required to support the PCB without pressing on glass, and place the bezel. Use PCB front lower-left as the X/Y origin. |
| SW1/SW2 Chanzon | Body X/Y; body-base-to-actuator-top; released and fully actuated heights; lead spacing, width and length; any underside protrusions | Required to put load on the body, give leads relief, set cap travel and prevent overtravel. The nominal 6 × 6 × 5 title is insufficient. |
| BAT1 EEMB | Exact label/revision; finished-pack outer dimensions; lead exit and length; connector dimensions; polarized connector face/pin order; pack charge/discharge limits | EEMB's store lists 20.5 × 32 × 5.3 mm, but this does not locate leads or establish that this delivered pack is identical. |
| J1 | Mated housing X/Y/Z, latch orientation, withdrawal distance and wire bend space | Prevents closing the case over the connector or making the pack impossible to disconnect. |
| USB cable | Plug-body X/Y/Z and insertion depth | Sets the case port, not just the metal USB shell. |

For hole positions, measure between centres rather than edges, or record hole
diameter and both edge distances so centres can be calculated. Record actual
measurements separately from clearance allowances. Do not change `evidence` to
`measured` merely because an envelope was edited in the browser.

## Small print-fit coupons

Generate with `make cad` (devenv v2). Files are under
`hardware/cad/out/coupons/` and included in the iteration ZIP. Print at 100% scale,
flat face on the bed, using the same material/profile intended for the case.
Record nozzle, layer height, material and any horizontal-expansion setting.
These test **lateral print fit only**, not switch mounting or electrical operation.

The notched edge identifies each opening: one notch, two notches, three notches.
Read the count at that opening; orientation is otherwise unimportant.

| Coupon | 1 notch | 2 notches | 3 notches |
| --- | --- | --- | --- |
| SWITCH_BODY_FIT | 6.1 × 6.1 opening | 6.3 × 6.3 | 6.5 × 6.5 |
| BUTTON_GUIDE_FIT | 10.2 × 8.2 opening | 10.4 × 8.4 | 10.6 × 8.6 |

Check the switch body in the first coupon without pushing on the actuator or
bending the leads. If its lead shape prevents insertion, stop and record that;
it identifies a needed slot, not a reason to force the part. The coupon has no
seat and cannot establish a final support height.

For the second coupon, also print one current CAP1 prototype from
`out/enclosure/`. Insert its smaller finger surface from beneath the coupon so
the wider flange stays underneath. Record which hole slides freely without
binding or excessive rattle. Gravity-only sliding does not establish reliable
return with the selected tactile switch.

| Result | 1 notch | 2 notches | 3 notches |
| --- | --- | --- | --- |
| Switch: no fit / snug / loose | pending | pending | pending |
| Cap: binds / slides / loose | pending | pending | pending |

## Electrical information still needed

Record the actual battery polarity and protection/charge specification, the
C3 board's power input arrangement, and any charger/regulator already on hand.
The power module remains unselected; its outline and connector access cannot be
frozen from a placeholder. The completed power circuit must handle USB-connected
operation without driving one supply into the other. Bench wiring remains USB-only.

## Design freeze

After these records are available: update component models, derive supports and
travel stops, route the harness, tune printed push-fit retention, rerun solid/clearance/access
checks, and print the first full assembly. Keep coupon results and assembled-fit
results distinct. Neither coupon constitutes approval of the current whole case.
