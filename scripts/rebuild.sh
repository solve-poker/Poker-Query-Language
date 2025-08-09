#!/usr/bin/env bash
set -euo pipefail

commands=(
  "cargo build"
  "cargo build --release"
  "cargo test"
  "cargo clippy"
)

pids=()
for cmd in "${commands[@]}"; do
  echo "📦 Starting: $cmd"
  bash -lc "$cmd" &
  pids+=($!)
done

exit_code=0
for pid in "${pids[@]}"; do
  if ! wait "$pid"; then
    echo "❌ A job (PID $pid) failed."
    exit_code=1
  fi
done

if [[ $exit_code -eq 0 ]]; then
  echo "✅ All jobs succeeded."
else
  echo "⚠️  At least one job failed."
fi

exit $exit_code
