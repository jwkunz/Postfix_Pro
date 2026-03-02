#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "error: wasm-pack is required but not installed." >&2
  echo "install: https://rustwasm.github.io/wasm-pack/installer/" >&2
  exit 1
fi

wasm-pack build --target web --out-dir pkg
cp index.html dist/index.html

echo "WASM build complete:"
echo "  - pkg/ web bindings generated"
echo "  - dist/index.html refreshed"
