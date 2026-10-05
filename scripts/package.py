"""Bundle the current iteration artifacts; never label them manufacturing-ready."""
from pathlib import Path
import hashlib
import json
import zipfile

root = Path(__file__).resolve().parents[1]
out = root / "hardware/cad/out"
report = json.loads((out / "fit-report.json").read_text())
package = out / "ceridwen-iteration-package.zip"
case_report = json.loads((out / "enclosure-report.json").read_text())
files = [root / "CHANGELOG.md", root / "devenv.nix", root / "devenv.yaml",
         root / "devenv.lock", root / "Cargo.lock", root / "Makefile"]
for folder in ["docs", "hardware/cad/src", "hardware/cad/scripts", "hardware/cad/tests",
               "hardware/cad/dist", "hardware/cad/public", "scripts", "ceridwen-core", "ceridwen-tui", "ceridwen-esp32"]:
    files.extend(p for p in (root / folder).rglob("*") if p.is_file()
                 and not any(x in p.parts for x in ["target", ".embuild"]))
files.extend(out.glob("*.json"))
files.extend(out.glob("*.csv"))
files.extend(out.glob("*.step"))
files.extend(out.glob("*.txt"))
files.extend(out.glob("*.svg"))
files.extend(out.glob("*.png"))
files.extend((out / "components").glob("*"))
files.extend((out / "enclosure").glob("*"))
files.extend((out / "coupons").glob("*"))
firmware = out / "firmware/ceridwen-esp32.elf"
if not firmware.is_file():
    raise RuntimeError("Run make build-esp32 before packaging")
files.extend((out / "firmware").glob("*"))
for name in ["Cargo.toml", "rust-toolchain.toml", "README.md", "AGENTS.md", ".gitignore", "hardware/cad/package.json",
             "hardware/cad/package-lock.json", "hardware/cad/tsconfig.json", "hardware/cad/vite.config.ts",
             "hardware/cad/playwright.config.ts", "hardware/cad/index.html"]:
    files.append(root / name)
manifest = {"status": "provisional iteration package", "manufacturing_ready": False,
            "firmware_built": True, "hardware_validated": False,
            "envelope_fit_pass": report["geometryPass"],
            "case_status": case_report["status"],
            "retention": "integral snap features", "screw_count": case_report["screwCount"],
            "push_pin_count": case_report["pushPinCount"],
            "printed_part_count": case_report["printedCount"],
            "case_interference_pass": json.loads((out / "enclosure-report.json").read_text())["interferencePass"],
            "case_motion_pass": json.loads((out / "enclosure-report.json").read_text())["motionPass"],
            "sha256": {}}
with zipfile.ZipFile(package, "w", compression=zipfile.ZIP_DEFLATED) as archive:
    for path in sorted(set(files)):
        if not path.is_file():
            raise RuntimeError(f"Missing package input: {path}")
        name = str(path.relative_to(root))
        data = path.read_bytes()
        archive.writestr(name, data)
        manifest["sha256"][name] = hashlib.sha256(data).hexdigest()
    archive.writestr("MANIFEST.json", json.dumps(manifest, indent=2))
print(package)
print(f"{len(manifest['sha256'])} files; provisional, not manufacturing-ready")
