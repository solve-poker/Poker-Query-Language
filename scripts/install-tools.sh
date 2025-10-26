#!/bin/bash
set -e

rustup component add rust-analyzer rust-src llvm-tools llvm-tools-preview
cargo binstall cargo-edit cargo-shear cargo-llvm-cov cargo-nextest --no-confirm
