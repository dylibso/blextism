export EXTISM_ENABLE_WASI_OUTPUT := "1"

repl:
    blender --log-level 3 --factory-startup -b -P repl.py

prepare:
  #!/bin/bash
  set -eou pipefail
  for file in $(find wat/ -name '*.wat'); do
    extless="${file%.wat}"
    extless=$(basename "$extless")
    wasm-tools print "$file" -w -o "wasm/$extless"
  done

_schema:
  #!/bin/bash
  if [ ! -e schema.json ] || [ blend.py -nt schema.json ]; then
    blender --log-level 3 --factory-startup -b -P blend.py | grep -v 'WARN (bpy.rna)' | grep -v 'Blender quit' | grep -v 'Device with name' | grep -ve '^Blender' > schema.json
  fi

build: _schema
  cargo build --release -p blender-extism-wasm-bindgen
  <schema.json target/release/blender-extism-wasm-bindgen > crates/pdk/src/bindings.rs
  cargo build --release --target wasm32-wasi -p plugin

run: build
  blender --log-level 3 --factory-startup -b -P run.py
