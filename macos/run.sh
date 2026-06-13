#!/usr/bin/env bash
set -euo pipefail

SPECK_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
[[ -f "$SPECK_DIR/Cargo.toml" ]] || SPECK_DIR="$(pwd)"
RUN_TS="$(date -u +%Y%m%dT%H%M%SZ)"
LOG_DIR="$SPECK_DIR/output/run-${RUN_TS}"

export RUSTFLAGS="-C target-cpu=native"

[[ -f "$SPECK_DIR/Cargo.toml" ]] || { echo "missing $SPECK_DIR/Cargo.toml"; exit 1; }
mkdir -p "$LOG_DIR"

PM_PID=""; SUDO_KEEPALIVE=""; CAFFEINATE_PID=""
RESTORE=()
cleanup() {
  trap '' INT TERM
  [[ -n "$PM_PID" ]] && { sudo kill "$PM_PID" 2>/dev/null || true; wait "$PM_PID" 2>/dev/null || true; }
  for c in ${RESTORE[@]+"${RESTORE[@]}"}; do eval "$c" >/dev/null 2>&1 || true; done
  kill "$SUDO_KEEPALIVE" "$CAFFEINATE_PID" 2>/dev/null || true
}
trap cleanup EXIT
trap 'exit 130' INT TERM

pm_start() {
  sudo powermetrics --samplers cpu_power,thermal -i 100 -o "$LOG_DIR/$1" >/dev/null 2>&1 &
  PM_PID=$!; sleep 1
}
pm_stop() {
  [[ -n "$PM_PID" ]] || return 0
  sudo kill "$PM_PID" 2>/dev/null || true; wait "$PM_PID" 2>/dev/null || true; PM_PID=""
}

sudo -v
( while true; do sudo -n true; sleep 50; kill -0 "$$" 2>/dev/null || exit; done ) & SUDO_KEEPALIVE=$!
sudo renice -n -20 -p "$$" >/dev/null 2>&1 || true
caffeinate -dimsu -w "$$" & CAFFEINATE_PID=$!

NAP="$(pmset -g 2>/dev/null | awk '/powernap/{print $2; exit}')"
[[ -n "$NAP" ]] && sudo pmset -a powernap 0 >/dev/null 2>&1 && RESTORE+=("sudo pmset -a powernap $NAP")
mdutil -s / 2>/dev/null | grep -qi enabled && sudo mdutil -a -i off >/dev/null 2>&1 && RESTORE+=("sudo mdutil -a -i on")
sudo tmutil disable >/dev/null 2>&1 && RESTORE+=("sudo tmutil enable")
WIFI_DEV="$(networksetup -listallhardwareports 2>/dev/null | awk '/Wi-Fi|AirPort/{getline; print $2; exit}')"
if [[ -n "$WIFI_DEV" ]]; then
  WIFI="$(networksetup -getairportpower "$WIFI_DEV" 2>/dev/null | awk '{print tolower($NF)}')"
  sudo networksetup -setairportpower "$WIFI_DEV" off >/dev/null 2>&1 && RESTORE+=("sudo networksetup -setairportpower $WIFI_DEV ${WIFI:-on}")
fi
if command -v blueutil >/dev/null 2>&1; then
  BT="$(blueutil -p 2>/dev/null)"
  blueutil -p 0 >/dev/null 2>&1 && RESTORE+=("blueutil -p ${BT:-1}")
fi

cd "$SPECK_DIR"

echo ">>> build"
cargo build --release && cargo bench --no-run
BIN="$SPECK_DIR/target/release/speck-probe"
[[ -x "$BIN" ]] || { echo "binary missing: $BIN"; exit 1; }

sleep 120

echo ">>> backend clock-drop probes"
"$BIN" sample search --force ./config/search.toml
for BK in Scalar Neon; do
  CFG="./config/search-${BK}.toml"
  sed "s/^backend_hint = .*/backend_hint = \"${BK}\"/" ./config/search.toml > "$CFG"
  pm_start "pm-search-${BK}.txt"
  "$BIN" search "$CFG"
  pm_stop
  sleep 120
done

echo ">>> cargo bench"
pm_start "pm-bench.txt"
cargo bench
pm_stop

echo ">>> extract-criterion"
"$BIN" extract-criterion \
    -i ./target/criterion/ \
    -o "$LOG_DIR/criterion.csv" \
    --clear-output

sleep 120

echo ">>> sample config"
"$BIN" sample benchmark --force ./config/benchmark.toml

echo ">>> speck-probe benchmark"
BENCH_DIR="$LOG_DIR/benchmark-parts"
mkdir -p "$BENCH_DIR"
SYS_CSV="$LOG_DIR/system.csv"

VERSIONS=()
while IFS= read -r v; do
  VERSIONS+=("$v")
done < <(grep -oE 'Speck[0-9]+_[0-9]+' ./config/benchmark.toml | awk '!seen[$0]++')
N=${#VERSIONS[@]}; I=0

for V in "${VERSIONS[@]}"; do
  I=$((I+1))
  VCFG="./config/benchmark-${V}.toml"
  sed "/^speck_versions = \[/,/^]/c\\
speck_versions = [\"$V\"]" ./config/benchmark.toml > "$VCFG"
  echo ">>> speck-probe benchmark [$I/$N] $V"
  pm_start "pm-system-${V}.txt"
  "$BIN" benchmark "$VCFG" -o "$BENCH_DIR/system-${V}.csv"
  pm_stop
  [[ $I -lt $N ]] && sleep 120
done

: > "$SYS_CSV"; HDR=0
for V in "${VERSIONS[@]}"; do
  f="$BENCH_DIR/system-${V}.csv"
  [[ -f "$f" ]] || continue
  if [[ $HDR -eq 0 ]]; then
    cat "$f" >> "$SYS_CSV"
    HDR=1
  else
    tail -n +2 "$f" >> "$SYS_CSV"
  fi
done

echo ">>> done — results in $LOG_DIR"
ls -la "$LOG_DIR"
