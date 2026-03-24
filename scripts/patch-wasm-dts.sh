#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <path>"
  exit 1
fi

find "$1" -name "*_bg.wasm.d.ts" | while read -r file; do
  if ! grep -q "export default WebAssembly.Module" "$file"; then
    echo "export default WebAssembly.Module;" >> "$file"
    echo "Patched: $file"
  fi
done
