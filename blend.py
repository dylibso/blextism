
try:
    import extism
except ImportError:
    from pip._internal import main as pipmain
    pipmain(['install', 'extism==1.0.0rc1'])
try:
    import typing_extensions
except ImportError:
    from pip._internal import main as pipmain
    pipmain(['install', 'typing_extensions'])

import builtins
import bpy
import json
import typing
import typing_extensions
from extism import host_fn, Plugin
from pprint import pprint

def main():
    with open(bpy.path.relpath("//extism.json"), "r") as f:
        manifest = json.loads(f.read())
    plugin = Plugin(manifest, wasi=True)
    plugin.call("main")

all_classes = set()
root = {}
for attr in dir(bpy.types):
    value = getattr(bpy.types, attr, None)
    if value is None:
        continue

    if hasattr(value, '__bases__'):
        all_classes.add(value)
        if len(value.__bases__) != 1:
            continue

        bases = []
        current = value
        while True:
            bases.append(current)
            if len(current.__bases__) == 1:
                current = current.__bases__[0]
            else:
                break

        bases.reverse()
        cursor = root
        for item in bases:
            next = cursor.get(item, {})
            cursor[item] = next
            cursor = next

# pprint(root[object][bpy.types.bpy_struct][bpy.types.Property])

def unpack_property_metadata(property_descriptor):
    output = {
        "identifier": property_descriptor.identifier,
        "description": property_descriptor.description,
        "type": property_descriptor.type,
        "unit": property_descriptor.unit,
        "subtype": property_descriptor.subtype,
        "is_required": property_descriptor.is_required,
        "is_runtime": property_descriptor.is_runtime,
        "is_output": property_descriptor.is_output,
        "is_never_none": property_descriptor.is_never_none,
    }
    match type(property_descriptor):
        case bpy.types.EnumProperty:
            # default
            # default_flag
            # enum_items_static (because plain "enum_items" crashes blender.)
            enum = []
            output["items"] = enum
            for (enum_name, desc) in property_descriptor.enum_items_static.items():
                enum.append({
                    "id": desc.identifier,
                    "name": desc.name,
                    "value": desc.value,
                    "description": desc.description,
                })

            return {"enum": output}

        case bpy.types.CollectionProperty:
            output["fixed_type"] = property_descriptor.fixed_type.identifier
            return {"collection": output}

        case bpy.types.PointerProperty:
            output["fixed_type"] = property_descriptor.fixed_type.identifier
            return {"pointer": output}

        case bpy.types.FloatProperty:
            output['hard_min'] = property_descriptor.hard_min
            output['hard_max'] = property_descriptor.hard_max
            output['soft_min'] = property_descriptor.soft_min
            output['soft_max'] = property_descriptor.soft_max
            if property_descriptor.is_array:
                output['default'] = [*property_descriptor.default_array]
                output['dimensions'] = [*property_descriptor.array_dimensions]
                output['length'] = property_descriptor.array_length

                return {"float[]": output}
            else:
                output['default'] = property_descriptor.default
                return {"float": output}

        case bpy.types.IntProperty:
            output['hard_min'] = property_descriptor.hard_min
            output['hard_max'] = property_descriptor.hard_max
            output['soft_min'] = property_descriptor.soft_min
            output['soft_max'] = property_descriptor.soft_max
            if property_descriptor.is_array:
                output['default'] = [*property_descriptor.default_array]
                output['dimensions'] = [*property_descriptor.array_dimensions]
                output['length'] = property_descriptor.array_length

                return {"int[]": output}
            else:
                output['default'] = property_descriptor.default
                return {"int": output}

        case bpy.types.BoolProperty:
            if property_descriptor.is_array:
                output['default'] = [*property_descriptor.default_array]
                output['dimensions'] = [*property_descriptor.array_dimensions]
                output['length'] = property_descriptor.array_length

                return {"bool[]": output}
            else:
                output['default'] = property_descriptor.default
                return {"bool": output}

        case bpy.types.StringProperty:
            output['length_max'] = property_descriptor.length_max
            output['default'] = property_descriptor.default
            return {"string": output}

        case _:
            raise Exception("Unexpected type")



def inspect_bpy_type(typ, output):
    props = {}
    methods = {}
    output["properties"] = props
    output["methods"] = methods
    for attr_name in vars(typ):
        if attr_name.startswith("__"):
            continue

        if attr_name == 'bl_rna':
            continue

        attr = getattr(typ, attr_name)

        methods[attr_name] = {"type": type(attr).__name__}

    if not hasattr(typ, 'bl_rna'):
        return

    for (prop_name, descriptor) in typ.bl_rna.properties.items():
        if prop_name == 'bl_rna':
            continue
        props[prop_name] = unpack_property_metadata(descriptor)

def inspect_recursive(tree, parent = object, output = []):
    for (cls, item) in tree.items():
        result = {"name": cls.__name__, "parent": parent.__name__}
        output.append(result)
        inspect_bpy_type(cls, result)
        inspect_recursive(item, cls, output)

    return output

classes = inspect_recursive(root[object])
print(json.dumps(classes, indent=2))

# print(dir(bpy.types.Object.bl_rna))
# print(bpy.data.objects['Cube'])
# print(bpy.types.bpy_prop_collection)
