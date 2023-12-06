
try:
    import extism
except ImportError:
    from pip._internal import main as pipmain
    pipmain(['install', 'extism==1.0.0rc1'])

import bpy
import json
from typing import Annotated, Optional
from extism import host_fn, Plugin, Json
from weakref import WeakValueDictionary
from dataclasses import dataclass

PTR_TO_BPY_STRUCTS = WeakValueDictionary() 

def encode_bpy_struct(bpy_struct: bpy.types.bpy_struct) -> int:
    ptr = bpy_struct.as_pointer()
    PTR_TO_BPY_STRUCTS[ptr] = bpy_struct
    return ptr

def decode_bpy_struct(data) -> Optional[bpy.types.bpy_struct]:
    ptr = data['@ptr']
    # typ = data['@type']

    target = PTR_TO_BPY_STRUCTS.get(ptr, default = None)
    if target is None:
        return None

    # TODO: if data[@type] is available, verify pointer type.
    return target


class UnknownPtr(Exception):
    ...

class InvalidTarget(Exception):
    ...


def _lift(value):
    if isinstance(value, list):
        return map(_lift, value)

    if isinstance(value, dict):
        if '@ptr' in value:
            return decode_bpy_struct(value)

        return dict(map(lambda xs: (xs[0], _lift(xs[1])), value.items()))

    return value


def _lower(value):
    if isinstance(value, list):
        return map(_lower, value)

    if isinstance(value, dict):
        return dict(map(lambda xs: (xs[0], _lower(xs[1])), value.items()))

    if isinstance(value, bpy.types.bpy_prop_collection):
        [encode_bpy_struct(member) for _, member in value.items()]
        return {'@ptr': encode_bpy_struct(value)}

    if isinstance(value, bpy.types.bpy_struct):
        return {'@ptr': encode_bpy_struct(value), '@type': value.rna_type.identifier}

    return value

@host_fn
def invoke_bpy_method(method: str, args: Json) -> Json:
    try:
        args = _lift(args)
        target = args.pop('self', None)
        star_args = args.pop('args', [])
        kwargs = args.pop('kwargs', {})
        if target is None:
            raise UnknownPtr()

        attr = getattr(target, method, None)
        if attr is None:
            raise InvalidTarget()

        return _lower(attr(*star_args, **kwargs) if callable(attr) else attr)
    except UnknownPtr:
        ...
    except InvalidTarget:
        ...


@host_fn
def bpy_data(which: str) -> Json:
    target = getattr(bpy.data, which, None)
    if target is None:
        return None

    return _lower(target)


@host_fn
def select_from_bpy_data_collection(which: str, name: str) -> Json:
    target = getattr(bpy.data, which, None)
    if target is None:
        return None

    return _lower(target.get(name))


def main():
    with open(bpy.path.relpath("//extism.json"), "r") as f:
        manifest = json.loads(f.read())
    plugin = Plugin(manifest, wasi=True)
    plugin.call("main")
