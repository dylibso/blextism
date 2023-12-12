
try:
    import extism
except ImportError:
    from pip._internal import main as pipmain
    pipmain(['install', 'extism==1.0.0rc1'])

import bpy
import json
from typing import Annotated, Any, Optional
from extism import host_fn, Plugin, Json, set_log_file
from weakref import WeakValueDictionary
from dataclasses import dataclass
import mathutils

PTR_TO_BPY_STRUCTS = dict() 

def encode_bpy_struct(bpy_struct: bpy.types.bpy_struct) -> int:
    ptr = id(bpy_struct)
    PTR_TO_BPY_STRUCTS[ptr] = bpy_struct
    return ptr

def decode_bpy_struct(data) -> Optional[bpy.types.bpy_struct]:
    ptr = data['@ptr']
    # typ = data['@type']

    target = PTR_TO_BPY_STRUCTS.get(ptr, None)
    if target is None:
        return None

    # TODO: if data[@type] is available, verify pointer type.
    return target


class UnknownPtr(Exception):
    ...

class InvalidTarget(Exception):
    ...


def _lift(value) -> Any:
    if isinstance(value, list):
        return [*map(_lift, value)]

    if isinstance(value, dict):
        if '@ptr' in value:
            return decode_bpy_struct(value)

        return dict(map(lambda xs: (xs[0], _lift(xs[1])), value.items()))

    return value


def _lower(value) -> Any:
    if isinstance(value, list):
        return [*map(_lower, value)]

    if isinstance(value, set):
        return [*map(_lower, value)]

    if isinstance(value, tuple):
        return tuple(map(_lower, value))

    if isinstance(value, dict):
        return dict(map(lambda xs: (xs[0], _lower(xs[1])), value.items()))

    if isinstance(value, mathutils.Vector):
        return value[:]

    if isinstance(value, bpy.types.bpy_prop_collection):
        [encode_bpy_struct(member) for _, member in value.items()]
        return {'@ptr': encode_bpy_struct(value)}

    if isinstance(value, bpy.types.bpy_struct):
        return {'@ptr': encode_bpy_struct(value), '@type': value.rna_type.identifier}

    return value

@host_fn(namespace = "chrisdickinson:blender/bpy")
def bpy_operator(mod: str, method: str, args: Annotated[dict, Json]) -> Annotated[dict | list, Json]:
    try:
        args_lifted = _lift(args)
        star_args = args_lifted.pop('args', []) or []
        kwargs = args_lifted.pop('kwargs', {}) or {}

        op_mod = getattr(bpy.ops, mod, None)
        if op_mod is None:
            raise InvalidTarget()

        attr = getattr(op_mod, method, None)
        if attr is None or not callable(attr):
            raise InvalidTarget()

        return _lower(attr(*star_args, **kwargs))
    except UnknownPtr:
        ...
    except InvalidTarget:
        ...

    return {}

@host_fn(namespace = "chrisdickinson:blender/bpy")
def bpy_callmethod(method: str, args: Annotated[dict, Json]) -> Annotated[dict | list, Json]:
    try:
        args_lifted = _lift(args)
        target = args_lifted.pop('self', None)
        star_args = args_lifted.pop('args', []) or []
        kwargs = args_lifted.pop('kwargs', {}) or {}
        if target is None:
            raise UnknownPtr()

        attr = getattr(target, method, None)
        if attr is None or not callable(attr):
            raise InvalidTarget()

        return _lower(attr(*star_args, **kwargs))
    except UnknownPtr:
        ...
    except InvalidTarget:
        ...

    return {}

@host_fn(namespace = "chrisdickinson:blender/bpy")
def bpy_getattr(attr_name: str, args: Annotated[dict, Json]) -> Annotated[dict | list, Json]:
    try:
        args = _lift(args)
        target = args.pop('self', None)
        if target is None:
            raise UnknownPtr()

        attr = getattr(target, attr_name, None)
        if attr is None:
            raise InvalidTarget()

        return _lower(attr)
    except UnknownPtr:
        ...
    except InvalidTarget:
        ...
    return {}

@host_fn(namespace = "chrisdickinson:blender/bpy")
def bpy_setattr(attr_name: str, args: Annotated[dict, Json]):
    try:
        args = _lift(args)
        target = args.pop('self', None)
        star_args = args.pop('args', [])
        if target is None:
            raise UnknownPtr()

        setattr(target, attr_name, *star_args)
    except UnknownPtr:
        ...
    except InvalidTarget:
        ...


def main():
    with open(bpy.path.abspath("//extism.json"), "r") as f:
        manifest = json.loads(f.read())

    set_log_file("stderr", "trace")

    bpy_data = ['actions', 'armatures', 'batch_remove', 'bl_rna', 'brushes', 'cache_files', 'cameras', 'collections', 'curves', 'filepath', 'fonts', 'grease_pencils', 'hair_curves', 'images', 'is_dirty', 'is_saved', 'lattices', 'libraries', 'lightprobes', 'lights', 'linestyles', 'masks', 'materials', 'meshes', 'metaballs', 'movieclips', 'node_groups', 'objects', 'orphans_purge', 'paint_curves', 'palettes', 'particles', 'pointclouds', 'rna_type', 'scenes', 'screens', 'shape_keys', 'sounds', 'speakers', 'temp_data', 'texts', 'textures', 'use_autopack', 'user_map', 'version', 'volumes', 'window_managers', 'workspaces', 'worlds']

    plugin = Plugin(manifest, wasi=True, config={
        'bpy.data': json.dumps({'context': encode_bpy_struct(bpy.context)} | dict(
            ((key, encode_bpy_struct(getattr(bpy.data, key))) for key in bpy_data)
        ))
    })

    plugin.call("example", "")
    bpy.ops.wm.save_as_mainfile(filepath="foo.blend")

if __name__ == '__main__':
    main()
