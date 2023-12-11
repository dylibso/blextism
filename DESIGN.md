# extism-blender

Refer to Blender objects and APIs within a Wasm guest module!

- Subclasses of `bpy_struct` are converted into pointers before being
  passed into Wasm
- These pointers are registered with a (weak?) map
    ptr -> (type, <object?>)
- Methods are registered once

---

why don't we see `.new` and `.remove` on BlendDataObjects?

---

- impl `__getitem__` for BlendDataTKTK
    - create `Index` impl? or `impl<T: bpy_prop_collect> Index for T`.
        - check to see if BlendDataObjects has multiple classes -- they do NOT
        - ~~allow for multiple parent classes OR~~ introduce a wrapper type
- implement `methods`
    - impl `new` and `remove`
- implement that one demo


collections:

you have a ptr to a trait
    which may have extra methods (`new`, `remove`)


instead of returning an existing trait, return the combination of the two `trait BpyPropCollectionBlendDataObjects : bpy_prop_collection + BlendDataObjects`
implement `bpy_prop_collection` methods there


trait BpyPropCollection : BlendDataObjects {
    type DestValue: bpy_struct;

    fn get(&self, usize) -> Self::DestValue;
}

trait BpyPropCollectionBlendDataObjects: BpyPropCollection + BlendDataObjects {}
impl BpyPropCollection<Item = Object> for BlendDataObjects;

two unique pieces of data
1. the container type (optionally)
2. the return type
    - we want to specialize the container type based on the return type

trait

----

- extras
- methods
- speed up generation
- ops



