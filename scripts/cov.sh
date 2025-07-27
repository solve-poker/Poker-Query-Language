#!/usr/bin/env bash

mkdir -p target/llvm-cov
cargo +nightly llvm-cov --no-report -j4
if [[ $# -gt 0 ]]; then
  cargo +nightly llvm-cov --no-report -j4 -- --ignored
fi

cargo +nightly llvm-cov report --lcov --output-path=target/llvm-cov/lcov
cargo +nightly llvm-cov report --html
