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

LOG_DIR="$SPECK_DIR/output/run-${RUN_TS}"
mkdir -p "$LOG_DIR"

push_out() {
  local f
  for f in "$@"; do
    [[ -e "$LOG_DIR/$f" ]] && cp -a "$LOG_DIR/$f" "$DATA_DIR/" 2>/dev/null || true
  done
  sync
}

TS_PID=""
ts_start() {
  modprobe msr 2>/dev/null || true
  taskset -c 0 turbostat --interval 0.1 --quiet \
    --show CoreTmp,PkgWatt,Bzy_MHz,Avg_MHz,CPU \
    --out "$LOG_DIR/$1" &
  TS_PID=$!
}
ts_stop() {
  [[ -n "$TS_PID" ]] && { kill "$TS_PID" 2>/dev/null || true; wait "$TS_PID" 2>/dev/null || true; }; TS_PID="";
}

boost() {
  local v="$1"
  for f in /sys/devices/system/cpu/cpufreq/boost \
           /sys/devices/system/cpu/amd_pstate/cpb_boost \
           /sys/devices/system/cpu/cpufreq/policy*/boost; do
    [[ -w "$f" ]] && echo "$v" > "$f" 2>/dev/null || true
  done
}

trap 'ts_stop; boost 1' EXIT

CORE1_CPUS="$(lscpu -p=CPU,CORE | awk -F, '!/^#/ && $2==1 {print $1}' | paste -sd,)"
[[ -n "$CORE1_CPUS" ]] || { echo "no CPUs found on physical core 1"; exit 1; }

cd "$SPECK_DIR"

echo ">>> build"
cargo --offline "${VENDOR_CFG[@]}" build --release && cargo --offline "${VENDOR_CFG[@]}" bench --no-run

BIN="$SPECK_DIR/target/release/speck-probe"
[[ -x "$BIN" ]] || { echo "binary missing: $BIN"; exit 1; }

sleep 120

echo ">>> backend clock-drop probes"
"$BIN" sample search --force ./config/search.toml
for BK in Scalar Sse2 Avx2 Avx512; do
  CFG="./config/search-${BK}.toml"
  sed "s/^backend_hint = .*/backend_hint = \"${BK}\"/" ./config/search.toml > "$CFG"
  boost 0
  ts_start "turbostat-search-${BK}.txt"
  "$BIN" search "$CFG"
  ts_stop
  boost 1
  push_out "turbostat-search-${BK}.txt"
  sleep 60
done

echo ">>> cargo bench speck"
for BK in scalar sse2 avx2 avx512; do
  ts_start "turbostat-speck-${BK}.txt"
  boost 0
  taskset -c "$CORE1_CPUS" chrt -r 99 cargo --offline "${VENDOR_CFG[@]}" bench speck/"${BK}"
  boost 1
  ts_stop
  push_out "turbostat-speck-${BK}.txt"
  sleep 60
done

echo ">>> cargo bench engine"
for BK in scalar sse2 avx2 avx512; do
  ts_start "turbostat-engine-${BK}.txt"
  boost 0
  taskset -c "$CORE1_CPUS" chrt -r 99 cargo --offline "${VENDOR_CFG[@]}" bench engine/"${BK}"
  boost 1
  ts_stop
  push_out "turbostat-engine-${BK}.txt"
  sleep 60
done

echo ">>> cargo bench system"
for BK in scalar sse2 avx2 avx512; do
  ts_start "turbostat-system-${BK}.txt"
  boost 0
  chrt -r 99 cargo --offline "${VENDOR_CFG[@]}" bench system/"${BK}"
  boost 1
  ts_stop
  push_out "turbostat-system-${BK}.txt"
  sleep 60
done

echo ">>> cargo bench compare"
boost 1
for VE in 32_64 48_72 64_96 128_128; do
  ts_start "turbostat-compare-${VE}.txt"
  chrt -r 99 cargo --offline "${VENDOR_CFG[@]}" bench compare/avx512/ecb/"${VE}"
  ts_stop
  push_out "turbostat-compare-${VE}.txt"
  sleep 60
done

echo ">>> extract-criterion"
"$BIN" extract-criterion \
    -i ./target/criterion/ \
    -o "$LOG_DIR/criterion_x86.csv" \
    --clear-output
push_out "criterion_x86.csv"

echo ">>> done — results in $DATA_DIR"
ls -la "$DATA_DIR"