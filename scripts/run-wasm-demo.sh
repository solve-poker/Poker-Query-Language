#!/usr/bin/env bash
set -euo pipefail

if ! command -v trunk &>/dev/null; then
  echo "Installing trunk..."
  cargo binstall trunk --no-confirm
fi

trunk serve --config ./open-pql-wasm/Trunk.toml
