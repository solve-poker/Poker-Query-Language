#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

cd "$SCRIPT_DIR/.." || exit 1

rm -rf ./target/coverage
mkdir -p ./target/coverage

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="./target/profraw/open-pql-%p-%m.profraw"
cargo nextest run
grcov . \
  --binary-path ./target/debug/ \
  -s . \
  -t html,lcov \
  --branch \
  --ignore-not-existing \
  --excl-line "(Readable|Writable)" \
  -o target/coverage

cd - || exit 1
echo "Coverage report generated."
