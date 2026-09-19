#!/usr/bin/env bash
set -euo pipefail

DEST="/home/adri/.config/Screeps/scripts/screeps.com/rust"

echo "Building Rust/WASM..."
rm -rf pkg

wasm-pack build \
  --target nodejs \
  --out-dir pkg \
  --out-name screeps

echo "Patching wasm loader for Screeps..."
python3 - <<'PY'
from pathlib import Path

path = Path("pkg/screeps.js")
text = path.read_text()

old = """const wasmPath = `${__dirname}/screeps_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);"""

new = """const wasmBytes = require('screeps_bg');"""

if old not in text:
    raise SystemExit(
        "Could not find expected wasm-pack loader in pkg/screeps.js"
    )

path.write_text(text.replace(old, new))
PY

echo "Copying to Screeps 'rust' branch..."
mkdir -p "$DEST"

cp main.js "$DEST/main.js"
cp pkg/screeps.js "$DEST/screeps.js"
cp pkg/screeps_bg.wasm "$DEST/screeps_bg.wasm"

echo
echo "Deployed to:"
echo "  $DEST"
echo
ls -lh \
  "$DEST/main.js" \
  "$DEST/screeps.js" \
  "$DEST/screeps_bg.wasm"
