#!/usr/bin/env bash

rustup toolchain install nightly

rustup component add rust-analyzer --toolchain nightly

rustup component add clippy --toolchain nightly

rustup component add rustfmt --toolchain nightly

rustup component add llvm-tools-preview --toolchain nightly

cargo install cargo-llvm-cov
