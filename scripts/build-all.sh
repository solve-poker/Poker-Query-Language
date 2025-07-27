#!/usr/bin/env bash

exit_status=0
cargo build || exit_status=$?
cargo build -r || exit_status=$?

cargo test || exit_status=$?

cargo +nightly clippy --fix || exit_status=$?
cargo +nightly fmt || exit_status=$?
#cargo +nightly bench -F benchmark || exit_status=$?
cargo doc || exit_status=$?

. ./scripts/cov.sh || exit_status=$?

exit "${exit_status:-0}"


# RUSTFLAGS="-C debuginfo=2 -C symbol-mangling-version=v0" cargo build --release
# perf record -F99 --call-graph dwarf cargo run --release
