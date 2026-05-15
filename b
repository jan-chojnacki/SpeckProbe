#!/bin/bash
set -euo pipefail

# katalog skryptu = root projektu speck
SPECK_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# rust perf flags
export RUSTFLAGS="-C target-cpu=native -C link-arg=-fuse-ld=mold"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang

exec cargo --offline \
  --config 'source.crates-io.replace-with="vendored-sources"' \
  --config "source.vendored-sources.directory=\"$SPECK_DIR/vendor\"" \
  "$@"