"""Verify archived assets after git lfs pull; do not trust unresolved LFS pointers."""
from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parents[1]/'hardware/snapshots'
count=0
for manifest in sorted(root.glob('*/manifest.json')):
 data=json.loads(manifest.read_text())
 for name,expected in data['sha256'].items():
  path=(manifest.parent/name).resolve()
  if not path.is_relative_to(manifest.parent.resolve()):raise SystemExit(f'Unsafe manifest path: {name}')
  if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest()!=expected:
   raise SystemExit(f'Asset mismatch: {path}. Run git lfs pull if it is still a pointer.')
  count+=1
 print(f"OK {manifest.parent.name}: {len(data['sha256'])} files")
print(f'Verified {count} archived files.')
