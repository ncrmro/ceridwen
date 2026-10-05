# Handheld bench wiring

Status: firmware pin assignment, not an as-built electrical acceptance record.
Verify the actual Teyleten board and Hosyond module labels before wiring.

[Open the printable wiring diagram](../../hardware/cad/public/wiring.svg). It is also available from the CAD workbench toolbar.

## USB-powered bench circuit

| Net | ESP32-C3 label | Destination |
| --- | --- | --- |
| Display power | 3V3 | OLED VCC |
| Common ground | GND | OLED GND and one terminal of each button |
| I2C data | GPIO4 | OLED SDA |
| I2C clock | GPIO5 | OLED SCL |
| Left button | GPIO0 | Other terminal of left Chanzon switch |
| Right button | GPIO1 | Other terminal of right Chanzon switch |

Firmware enables internal pull-ups on both button inputs. Each switch connects
its input to ground; both pressed together select. The two switch pins must land
on electrically separate breadboard strips. A board straddling the centre channel
must not short opposite pin rows. Verify grid alignment and rail continuity;
these are not established by the CAD proxies.

Use the OLED's printed pin labels rather than another SSD1306 board's physical
pin order. Keep the four display wires short. Keep BOOT/RESET and USB accessible.
GPIO8 LED handling is inherited from the original setup; confirm actual polarity
and behavior on the selected revision.

## Battery circuit remains open

The EEMB 502030 250 mAh battery with JST 2.0 mm connector is selected. The charger,
regulator, USB/battery isolation and power-off scheme remain unresolved. PWR2 in
CAD is reserved space, not an electrical design.

Select the power assembly against the pack's actual charge/discharge ratings,
verify connector polarity/protection, and record its schematic and installed
dimensions. Do not connect raw battery voltage to 3V3 or assume the C3 USB port
charges a cell. Test USB insertion/removal on the selected circuit before case
integration.

## Physical acceptance

Test each button and both together, every displayed choice, retry/next behavior,
USB flashing/reset access and display visibility. Measure active/sleep current
and runtime after power integration. These checks remain pending; compilation
and host simulation do not establish hardware behavior.

## Human assembly sequence

1. Keep the battery unplugged and remove the USB cable. Identify the printed
   pin labels on the actual boards. The diagram deliberately does not invent
   a physical pin order or breadboard row numbers.
2. With a continuity meter, identify the breadboard five-hole strips, centre
   channel and all four rail sections. Mark one verified section GND. Join
   sections only when the harness needs it; record every added jumper.
3. Seat the C3 across the centre channel only after checking that its header
   spacing fits without bending pins and no opposite pins share a strip.
4. Install the four OLED connections in the diagram, checking each by label.
5. Insert each two-pin switch into separate electrically isolated strips.
   Verify open circuit when released and continuity when pressed, then connect
   GPIO0 to SW1 and GPIO1 to SW2. Connect their remaining pins to common GND.
6. Check every connection against the table and check for a persistent short
   between 3V3 and GND before plugging USB in. Inspect for shifted headers.
7. Power by USB, verify the display, each button, and both-button selection.
   If the display is blank, inspect labels and I2C address before changing wires.

### Harness cut list

| Wire | From | To | Suggested color |
| --- | --- | --- | --- |
| W01 | U1 3V3 | DISP1 VCC | Red |
| W02 | U1 GPIO4 | DISP1 SDA | Blue |
| W03 | U1 GPIO5 | DISP1 SCL | Yellow |
| W04 | U1 GND strip | DISP1 GND | Black |
| W05 | U1 GPIO0 | SW1 terminal A | Green |
| W06 | SW1 terminal B | U1 GND strip | Black |
| W07 | U1 GPIO1 | SW2 terminal A | Violet |
| W08 | SW2 terminal B | U1 GND strip | Black |

A/B are arbitrary names for the two non-polarized switch terminals. Use
breadboard-compatible jumper terminations; the OLED end depends on its actual
installed header. Cut lengths remain pending routed geometry and a trial
assembly. Include bends, strain relief and enough slack to open the lid without
pulling contacts. This is a complete logical USB bench connection list, not a
completed battery harness or an as-built row-coordinate drawing.

## Connection to the case design

The diagram describes the electrical nets for the USB bench. The case study now
places the two switches on an elevated support above the power module, rather
than inserting their leads directly into the breadboard. Their four connections
(W05–W08) therefore require insulated pigtails with strain relief and appropriate
terminations. Lead geometry, solder access and support design remain pending.
Do not assume the bench's direct-insertion procedure establishes case fit.

The battery and power-module reservation also moved above the breadboard. Route
their future harness through the carrier's internal passages, not around the
outside of the board. The battery remains disconnected until the supply circuit
is selected and validated.
