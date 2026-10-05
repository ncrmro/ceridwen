"""Collect the exact ELF and ESP-IDF outputs reported by this Cargo build."""
import json
from pathlib import Path
import shutil
import sys

root = Path(__file__).resolve().parents[1]
events = [json.loads(line) for line in Path(sys.argv[1]).read_text().splitlines() if line.startswith('{')]
elf = next(Path(e['executable']) for e in events if e.get('reason') == 'compiler-artifact'
           and e.get('target', {}).get('name') == 'ceridwen-esp32' and e.get('executable'))
idf = next(Path(e['out_dir']) for e in events if e.get('reason') == 'build-script-executed'
           and 'esp-idf-sys' in e.get('package_id', ''))
dest = root / 'hardware/cad/out/firmware'
dest.mkdir(parents=True, exist_ok=True)
for source, name in [(elf, 'ceridwen-esp32.elf'),
                     (idf / 'build/bootloader/bootloader.bin', 'bootloader.bin'),
                     (idf / 'build/partition_table/partition-table.bin', 'partition-table.bin')]:
    if not source.is_file():
        raise RuntimeError(f'Missing firmware artifact: {source}')
    shutil.copyfile(source, dest / name)
(dest / 'README.txt').write_text(
    'ESP32-C3 / ESP-IDF v5.2 / nightly-2026-04-01\n'
    'Built, not flashed or hardware-validated.\n'
    'From this repository: make upload-esp32\n'
    'ELF, bootloader and partition table are collected from the same Cargo build.\n'
    'Do not flash to another chip/board without verifying its configuration.\n')
print(f'Collected firmware artifacts in {dest}')
