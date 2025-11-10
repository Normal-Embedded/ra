#!/usr/bin/env bash
set -euo pipefail

# --- config ---
DEV="${DEV:-R7FA4M1AB}"           # override by: DEV=RA6M3 ./gen.sh
TRANSFORM="${TRANSFORM:-ra.yaml}" # your transforms file
SVD_DIR="${SVD_DIR:-./svd}"       # where SVDs live
OUT_BASE="${OUT_BASE:-./out}"     # chiptool default; leave unless you pass --out-dir
BACKTRACE="${BACKTRACE:-full}"    # or "1" if you prefer shorter

# --- derive names ---
LOWER="$(printf '%s' "$DEV" | tr 'A-Z' 'a-z')"
CRATE_DIR="out-$LOWER"
mkdir -p "$CRATE_DIR/src"

SVD_PATH="$SVD_DIR/$DEV.svd"
[ -f "$SVD_PATH" ] || { echo "ERR: SVD not found: $SVD_PATH"; exit 1; }
[ -f "$TRANSFORM" ] || { echo "ERR: transform not found: $TRANSFORM"; exit 1; }

echo "==> Generating PAC from $SVD_PATH using $TRANSFORM …"

# If your chiptool supports an explicit out dir, uncomment and use it:
#   --out-dir "$OUT_BASE"
# (Otherwise chiptool will write into ./out by default.)
RUST_BACKTRACE="$BACKTRACE" RUST_LOG=debug \
chiptool generate --svd "$SVD_PATH" --transform "$TRANSFORM" || {
  echo "ERR: chiptool failed (see logs above)."
  exit 1
}

# Figure out where chiptool wrote lib.rs
CANDIDATES=(
  "$OUT_BASE/src/lib.rs"       # common default (./out/src/lib.rs)
  "./src/lib.rs"           # fallback if OUT_BASE differs
)
FOUND=""
for p in "${CANDIDATES[@]}"; do
  if [ -f "$p" ]; then FOUND="$p"; break; fi
done

if [ -z "$FOUND" ]; then
  echo "ERR: lib.rs not produced (chiptool likely panicked before writing)."
  echo "Tip: rerun with: RUST_BACKTRACE=full RUST_LOG=debug chiptool …"
  exit 1
fi

cp "$FOUND" "$CRATE_DIR/src/lib.rs"
echo "==> Wrote $CRATE_DIR/src/lib.rs"

if command -v rustfmt >/dev/null 2>&1; then
  rustfmt "$CRATE_DIR/src/lib.rs" || true
fi

echo "✅ Done: $CRATE_DIR/src/lib.rs"