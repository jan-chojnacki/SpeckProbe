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
  sleep 120
done

sleep 120

echo ">>> cargo bench"
ts_start "turbostat-bench.txt"
boost 0
taskset -c "$CORE1_CPUS" chrt -r 99 cargo --offline "${VENDOR_CFG[@]}" bench
boost 1
ts_stop
push_out "turbostat-bench.txt"

echo ">>> extract-criterion"
"$BIN" extract-criterion \
    -i ./target/criterion/ \
    -o "$LOG_DIR/criterion.csv" \
    --clear-output
push_out "criterion.csv"

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
  pm_start "turbostat-system-${V}.txt"
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

sleep 120

echo ">>> speck-compare"
CMP_CFG="./config/benchmark-compare.toml"
sed -e '/^cipher_modes = \[/,/^]/c\
cipher_modes = ["Ecb"]' \
    -e '/^backend_hints = \[/,/^]/c\
backend_hints = ["Auto"]' \
    -e '/^suffix_bytes_values = \[/,/^]/c\
suffix_bytes_values = [2]' \
    ./config/benchmark.toml > "$CMP_CFG"

CMP_DIR="$LOG_DIR/compare-parts"
mkdir -p "$CMP_DIR"
CMP_CSV="$LOG_DIR/compare.csv"

CMP_VERSIONS=(Speck32_64 Speck48_72 Speck64_96 Speck128_128)
CN=${#CMP_VERSIONS[@]}; CI=0

for V in "${CMP_VERSIONS[@]}"; do
  CI=$((CI+1))
  VCFG="./config/benchmark-compare-${V}.toml"
  sed "/^speck_versions = \[/,/^]/c\\
speck_versions = [\"$V\"]" "$CMP_CFG" > "$VCFG"
  echo ">>> speck-compare [$CI/$CN] $V"
  pm_start "turbostat-compare-${V}.txt"
  "$BIN" benchmark "$VCFG" -o "$CMP_DIR/compare-${V}.csv"
  pm_stop
  [[ $CI -lt $CN ]] && sleep 120
done

: > "$CMP_CSV"; CHDR=0
for V in "${CMP_VERSIONS[@]}"; do
  f="$CMP_DIR/compare-${V}.csv"
  [[ -f "$f" ]] || continue
  if [[ $CHDR -eq 0 ]]; then
    cat "$f" >> "$CMP_CSV"
    CHDR=1
  else
    tail -n +2 "$f" >> "$CMP_CSV"
  fi
done


echo ">>> done — results in $DATA_DIR"
ls -la "$DATA_DIR"