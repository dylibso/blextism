export EXTISM_ENABLE_WASI_OUTPUT := "1"

repl:
    blender --log-level 3 --factory-startup -b -P repl.py

_schema:
  #!/bin/bash
  if [ ! -e schema.json ] || [ genschema.py -nt schema.json ]; then
    blender --log-level 3 --factory-startup -b -P genschema.py | grep -v 'WARN (bpy.rna)' | grep -v 'Blender quit' | grep -v 'Device with name' | grep -ve '^Blender' > schema.json
  fi

build: _schema
  #!/bin/bash
  cargo build --release -p blextism-bindgen
  <schema.json target/release/blextism-bindgen > target/bindings.rs
  if [ -e crates/pdk/src/bindings.rs ]; then
    lhs=$(shasum target/bindings.rs | cut -d' ' -f1)
    rhs=$(shasum crates/pdk/src/bindings.rs | cut -d ' ' -f1)
    echo "$lhs" "$rhs"
    if [ "$lhs" != "$rhs" ]; then
      cp target/bindings.rs crates/pdk/src/bindings.rs
    fi
  else
    mv target/bindings.rs crates/pdk/src/bindings.rs
  fi
  cargo build --release --target wasm32-wasi -p plugin

run: build
  rm -f *.blend
  blender --log-level 3 --factory-startup -b -P run.py
