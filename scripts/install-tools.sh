#!/bin/bash
set -e

rustup component add rust-analyzer rust-src llvm-tools
cargo binstall cargo-edit grcov --no-confirm
