#!/bin/bash
set -euo pipefail

mkdir -p ./output

rm -f target/debug/deps/*.ll

cargo rustc --lib -- --emit=llvm-ir -C opt-level=0 -C debuginfo=0 -C codegen-units=1 2>/dev/null
cp target/debug/deps/speck_probe-*.ll /tmp/lib_o0.ll

cargo rustc --bin measure_memory -- --emit=llvm-ir -C opt-level=0 -C debuginfo=0 -C codegen-units=1 2>/dev/null
cp target/debug/deps/measure_memory-*.ll /tmp/bin_o0.ll

cat /tmp/lib_o0.ll /tmp/bin_o0.ll > ./output/o0.ll

rm -f target/debug/deps/*.ll

cargo rustc --lib -- --emit=llvm-ir -C opt-level=3 -C debuginfo=0 -C codegen-units=1 2>/dev/null
cp target/debug/deps/speck_probe-*.ll /tmp/lib_o3.ll

cargo rustc --bin measure_memory -- --emit=llvm-ir -C opt-level=3 -C debuginfo=0 -C codegen-units=1 2>/dev/null
cp target/debug/deps/measure_memory-*.ll /tmp/bin_o3.ll

cat /tmp/lib_o3.ll /tmp/bin_o3.ll > ./output/o3.ll