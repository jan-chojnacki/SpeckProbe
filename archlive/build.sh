#!/bin/bash
set -euo pipefail

WORK_DIR="$(pwd)/work"
OUT_DIR="$(pwd)/out"
PROFILE_DIR="$(pwd)/iso"
REPO_ROOT="$(cd .. && pwd)"
SPECK_DST="$PROFILE_DIR/airootfs/root/speck"

echo ">>> vendoring crates"
pushd "$REPO_ROOT" > /dev/null
cargo fetch --locked 2>/dev/null || cargo fetch
cargo vendor --versioned-dirs --quiet > vendor-config-snippet.toml
popd > /dev/null

echo ">>> staging repo into airootfs"
sudo rm -rf "$SPECK_DST"
mkdir -p "$SPECK_DST/.cargo"

cp -a "$REPO_ROOT/Cargo.toml"  "$SPECK_DST/"
cp -a "$REPO_ROOT/Cargo.lock"  "$SPECK_DST/"
cp -a "$REPO_ROOT/src"         "$SPECK_DST/"
cp -a "$REPO_ROOT/benches"     "$SPECK_DST/"
cp -a "$REPO_ROOT/vendor"      "$SPECK_DST/"

cp -a "$REPO_ROOT/.cargo/config.toml" "$SPECK_DST/.cargo/config.toml"
cat "$REPO_ROOT/vendor-config-snippet.toml" >> "$SPECK_DST/.cargo/config.toml"

echo ">>> offline check"
( cd "$SPECK_DST" && cargo fetch --offline )

echo ">>> building ISO"
sudo rm -rf "$WORK_DIR"
sudo mkarchiso -v -w "$WORK_DIR" -o "$OUT_DIR" "$PROFILE_DIR"
ls -la "$OUT_DIR/"

echo ">>> cleanup"
sudo rm -rf "$SPECK_DST"
rm -f "$REPO_ROOT/vendor-config-snippet.toml"