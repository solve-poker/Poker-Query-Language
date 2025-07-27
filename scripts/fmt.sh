#!/usr/bin/env bash

cargo +nightly clippy --fix
cargo +nightly fmt
