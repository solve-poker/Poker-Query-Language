#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

cd "$SCRIPT_DIR/.." || exit 1

cargo clippy --fix --allow-dirty
cargo clippy --fix --tests --allow-dirty
cargo +nightly fmt

cd - || exit 1
