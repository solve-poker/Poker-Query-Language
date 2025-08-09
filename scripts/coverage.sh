#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

cd "$SCRIPT_DIR/.." || exit 1

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="$SCRIPT_DIR/../target/profraw/open-pql-%p-%m.profraw"
cargo test
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o target/coverage/

cd - || exit 1
echo "Coverage report generated."
