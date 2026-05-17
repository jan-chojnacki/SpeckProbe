#!/bin/bash
export RUSTFLAGS="-C link-arg=-fuse-ld=mold -C target-cpu=native"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang