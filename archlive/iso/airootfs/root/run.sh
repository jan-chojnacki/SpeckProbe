#!/bin/bash
set -euo pipefail

SPECK_DIR=/root/speck
DATA_ROOT=/mnt/data
RUN_TS="$(date -u +%Y%m%dT%H%M%SZ)"
DATA_DIR="${DATA_ROOT}/run-${RUN_TS}"

export RUSTFLAGS="-C target-cpu=native -C link-arg=-fuse-ld=mold"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang

VENDOR_CFG=(
  --config 'source.crates-io.replace-with="vendored-sources"'
  --config "source.vendored-sources.directory=\"$SPECK_DIR/vendor\""
)

[[ -d "$SPECK_DIR" ]] || { echo "missing $SPECK_DIR"; exit 1; }
mountpoint -q "$DATA_ROOT" || mount "$DATA_ROOT" || {
  echo "DATA partition not mounted at $DATA_ROOT"; exit 1; }
mkdir -p "$DATA_DIR"

boost() {
  local v="$1"
  for f in /sys/devices/system/cpu/cpufreq/boost \
           /sys/devices/system/cpu/amd_pstate/cpb_boost \
           /sys/devices/system/cpu/cpufreq/policy*/boost; do
    [[ -w "$f" ]] && echo "$v" > "$f" 2>/dev/null || true
  done
}

trap 'boost 1' EXIT

CORE1_CPUS="$(lscpu -p=CPU,CORE | awk -F, '!/^#/ && $2==1 {print $1}' | paste -sd,)"
[[ -n "$CORE1_CPUS" ]] || { echo "no CPUs found on physical core 1"; exit 1; }

cd "$SPECK_DIR"

echo ">>> build"
cargo --offline "${VENDOR_CFG[@]}" build --release

BIN="$SPECK_DIR/target/release/speck-probe"
[[ -x "$BIN" ]] || { echo "binary missing: $BIN"; exit 1; }

echo ">>> cargo bench --no-run"
cargo --offline "${VENDOR_CFG[@]}" bench --no-run

echo ">>> cargo bench"
boost 0
taskset -c "$CORE1_CPUS" chrt -r 99 cargo --offline "${VENDOR_CFG[@]}" bench
boost 1

echo ">>> extract-criterion"
"$BIN" extract-criterion \
    -i ./target/criterion/ \
    -o ./output/criterion.csv \
    --clear-output
mv ./output/criterion.csv "$DATA_DIR/criterion.csv"

echo ">>> sample config"
"$BIN" sample benchmark --force ./config/benchmark.toml

echo ">>> speck-probe benchmark"
mkdir -p ./output
boost 0
"$BIN" benchmark ./config/benchmark.toml -o ./output/system.csv
boost 1
mv ./output/system.csv "$DATA_DIR/system.csv"

sync
echo ">>> done — results in $DATA_DIR"
ls -la "$DATA_DIR"