#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

cd "$SCRIPT_DIR/../openpql-runner" || exit 1

cargo expand functions

cd - || exit 1
