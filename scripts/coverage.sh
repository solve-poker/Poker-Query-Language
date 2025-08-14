#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

cd "$SCRIPT_DIR/.." || exit 1

cargo llvm-cov nextest --no-report
cargo llvm-cov report --html --show-instantiations --output-dir "./target/coverage/"
cargo llvm-cov report --lcov --output-path "./target/coverage/lcov"

cd - || exit 1
echo "Coverage report generated."
