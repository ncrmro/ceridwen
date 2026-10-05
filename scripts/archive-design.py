"""Archive generated CAD and captures without overwriting an existing milestone."""
from pathlib import Path
import argparse, datetime, hashlib, json, shutil, zipfile
p=argparse.ArgumentParser()
p.add_argument('milestone');p.add_argument('--cad',type=Path,default=Path('hardware/cad'))
p.add_argument('--provenance',required=True);p.add_argument('--original-observed-at')
a=p.parse_args();root=Path(__file__).resolve().parents[1]
dest=root/'hardware/snapshots'/a.milestone
if dest.exists(): raise SystemExit(f'Refusing to overwrite {dest}')
cad=a.cad.resolve();out=cad/'out';report=json.loads((out/'enclosure-report.json').read_text())
if not all(report[k] for k in ['interferencePass','motionPass','footprintPass','headerGridPass']):raise SystemExit('CAD checks must pass before archiving')
capture=json.loads((out/'captures/capture.json').read_text())
if capture['parameterSha256']!=hashlib.sha256((out/'parameters.json').read_bytes()).hexdigest():raise SystemExit('Capture parameters do not match model exports')
dest.mkdir(parents=True)
for directory in ['components','enclosure','captures']:
 shutil.copytree(out/directory,dest/directory)
for name in ['parameters.json','assembly.json','component-metadata.json','enclosure-report.json','fit-report.json','harness.json','bom.csv','assembly-with-case.step']:
 shutil.copy2(out/name,dest/name)
for name in ['snaps.json','push-pins.json','fasteners.json']:
 if (out/name).exists():shutil.copy2(out/name,dest/name)
with zipfile.ZipFile(dest/'cad-source.zip','w',zipfile.ZIP_DEFLATED) as z:
 for directory in ['src','scripts']:
  for f in sorted((cad/directory).rglob('*')):
   if f.is_file():z.write(f,f.relative_to(cad))
 for name in ['package.json','package-lock.json','tsconfig.json','vite.config.ts','index.html','public/wiring.svg']:
  z.write(cad/name,name)
files={str(f.relative_to(dest)):hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted(dest.rglob('*')) if f.is_file()}
manifest={'milestone':a.milestone,'archivedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'originalObservedAt':a.original_observed_at,'provenance':a.provenance,'capture':capture,'printedParts':report['printedCount'],'screws':report.get('screwCount',0),'pushPins':report.get('pushPinCount',0),'manufacturingReady':False,'sha256':files}
(dest/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(f'{dest}: {len(files)} files')
