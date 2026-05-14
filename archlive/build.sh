#!/bin/bash
set -euo pipefail

WORK_DIR="$(pwd)/work"
OUT_DIR="$(pwd)/out"
PROFILE_DIR="$(pwd)/myiso"

sudo rm -rf "$WORK_DIR"
sudo mkarchiso -v -w "$WORK_DIR" -o "$OUT_DIR" "$PROFILE_DIR"
ls -la "$OUT_DIR/"