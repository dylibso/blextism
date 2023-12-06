# extism-blender

Refer to Blender objects and APIs within a Wasm guest module!

- Subclasses of `bpy_struct` are converted into pointers before being
  passed into Wasm
- These pointers are registered with a (weak?) map
    ptr -> (type, <object?>)
- Methods are registered once

---

why don't we see `.new` and `.remove` on BlendDataObjects?
