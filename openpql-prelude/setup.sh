#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

# export RUSTUP_TOOLCHAIN=nightly
# source <(cargo +nightly llvm-cov show-env --export-prefix)
# cargo +nightly llvm-cov nextest --no-report
# cargo +nightly llvm-cov report --html --show-instantiations --output-dir "../target/coverage/"
# cargo +nightly llvm-cov report --lcov --output-path "../target/coverage/lcov"
# uv run -- pytest tests
# uv run -- maturin develop --uv

cd "$SCRIPT_DIR" || exit 1

uv venv --python 3.13
uv sync -U
uv run python -m maturin_import_hook site install

cd - || exit 1
