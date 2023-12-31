import bpy
import json

all_classes = set()
root = {}
for attr in dir(bpy.types):
    value = getattr(bpy.types, attr, None)
    if value is None:
        continue

    if hasattr(value, "__bases__"):
        all_classes.add(value)
        if len(value.__bases__) == 0:
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

POINTERS = []


def unpack_property_metadata(property_descriptor):
    output = {
        "identifier": property_descriptor.identifier,
        "description": property_descriptor.description,
        "type": property_descriptor.type,
        "unit": property_descriptor.unit,
        "name": property_descriptor.name,
        "subtype": property_descriptor.subtype,
        "is_argument_optional": property_descriptor.is_argument_optional,
        "is_required": property_descriptor.is_required,
        "is_runtime": property_descriptor.is_runtime,
        "is_output": property_descriptor.is_output,
        "is_never_none": property_descriptor.is_never_none,
    }
    match type(property_descriptor):
        case bpy.types.EnumProperty:
            # use "enum_items_static" because plain "enum_items" crashes blender.
            enum = []
            output["items"] = enum
            for desc in property_descriptor.enum_items_static.values():
                enum.append(
                    {
                        "id": desc.identifier,
                        "name": desc.name,
                        "value": desc.value,
                        "description": desc.description,
                    }
                )

            return {"enum": output}

        case bpy.types.CollectionProperty:
            output["fixed_type"] = property_descriptor.fixed_type.identifier
            if (
                hasattr(property_descriptor, "srna")
                and property_descriptor.srna is not None
            ):
                output["collection"] = property_descriptor.srna.identifier
            POINTERS.append(output)
            return {"collection": output}

        case bpy.types.PointerProperty:
            output["fixed_type"] = property_descriptor.fixed_type.identifier
            POINTERS.append(output)
            return {"pointer": output}

        case bpy.types.FloatProperty:
            output["hard_min"] = property_descriptor.hard_min
            output["hard_max"] = property_descriptor.hard_max
            output["soft_min"] = property_descriptor.soft_min
            output["soft_max"] = property_descriptor.soft_max
            if property_descriptor.is_array:
                output["default"] = [*property_descriptor.default_array]
                output["dimensions"] = [*property_descriptor.array_dimensions]
                output["length"] = property_descriptor.array_length

                return {"float[]": output}
            else:
                output["default"] = property_descriptor.default
                return {"float": output}

        case bpy.types.IntProperty:
            output["hard_min"] = property_descriptor.hard_min
            output["hard_max"] = property_descriptor.hard_max
            output["soft_min"] = property_descriptor.soft_min
            output["soft_max"] = property_descriptor.soft_max
            if property_descriptor.is_array:
                output["default"] = [*property_descriptor.default_array]
                output["dimensions"] = [*property_descriptor.array_dimensions]
                output["length"] = property_descriptor.array_length

                return {"int[]": output}
            else:
                output["default"] = property_descriptor.default
                return {"int": output}

        case bpy.types.BoolProperty:
            if property_descriptor.is_array:
                output["default"] = [*property_descriptor.default_array]
                output["dimensions"] = [*property_descriptor.array_dimensions]
                output["length"] = property_descriptor.array_length

                return {"bool[]": output}
            else:
                output["default"] = property_descriptor.default
                return {"bool": output}

        case bpy.types.StringProperty:
            output["length_max"] = property_descriptor.length_max
            output["default"] = property_descriptor.default
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

        if attr_name == "bl_rna":
            continue

        attr = getattr(typ, attr_name)

        if callable(attr):
            methods[attr_name] = {"type": type(attr).__name__}

    if not hasattr(typ, "bl_rna"):
        return

    for prop_name, descriptor in typ.bl_rna.functions.items():
        if prop_name == "bl_rna":
            continue

        methods[prop_name] = {
            "type": "rna",
            "item": {
                "description": descriptor.description,
                "use_self": descriptor.use_self,
                "use_self_type": descriptor.use_self,
                "parameters": [
                    *map(
                        lambda xs: unpack_property_metadata(xs),
                        descriptor.parameters,
                    )
                ],
            },
        }

    for prop_name, descriptor in typ.bl_rna.properties.items():
        if prop_name == "bl_rna":
            continue
        props[prop_name] = unpack_property_metadata(descriptor)


def disambiguate_render_property(output):
    # RenderEngine defines "render" as both a property (for RenderSettings) AND a
    # method (".render()"), so we disambiguate here.
    render_prop = output["properties"].pop("render", None)
    if render_prop is not None:
        render_prop["pointer"]["identifier"] = "render_settings"
        output["properties"]["render_settings"] = render_prop


def add_context_properties(output):
    # extracted from https://github.com/blender/blender/blob/9c0bffcc89f174f160805de042b00ae7c201c40b/doc/python_api/sphinx_doc_gen.py#L1161
    # Bone: props commented out until we get Bone types included
    # in schema generation (they have multiple bases so they're omitted right now)
    items = {
        # context_member: (RNA type, is_collection)
        "active_action": ("Action", False),
        "active_annotation_layer": ("GPencilLayer", False),
        # "active_bone": ("EditBone", False),
        "active_file": ("FileSelectEntry", False),
        # "active_gpencil_frame": ("GreasePencilLayer", True),
        "active_gpencil_layer": ("GPencilLayer", True),
        "active_node": ("Node", False),
        "active_object": ("Object", False),
        "active_operator": ("Operator", False),
        # "active_pose_bone": ("PoseBone", False),
        "active_sequence_strip": ("Sequence", False),
        "active_editable_fcurve": ("FCurve", False),
        "active_nla_strip": ("NlaStrip", False),
        "active_nla_track": ("NlaTrack", False),
        "annotation_data": ("GreasePencil", False),
        "annotation_data_owner": ("ID", False),
        "armature": ("Armature", False),
        "asset_library_reference": ("AssetLibraryReference", False),
        # "bone": ("Bone", False),
        "brush": ("Brush", False),
        "camera": ("Camera", False),
        "cloth": ("ClothModifier", False),
        "collection": ("Collection", False),
        "collision": ("CollisionModifier", False),
        "curve": ("Curve", False),
        "dynamic_paint": ("DynamicPaintModifier", False),
        # "edit_bone": ("EditBone", False),
        "edit_image": ("Image", False),
        "edit_mask": ("Mask", False),
        "edit_movieclip": ("MovieClip", False),
        "edit_object": ("Object", False),
        "edit_text": ("Text", False),
        # "editable_bones": ("EditBone", True),
        # "editable_gpencil_layers": ("GPencilLayer", True),
        "editable_gpencil_strokes": ("GPencilStroke", True),
        "editable_objects": ("Object", True),
        "editable_fcurves": ("FCurve", True),
        # "fluid": ("FluidSimulationModifier", False),
        "gpencil": ("GreasePencil", False),
        "gpencil_data": ("GreasePencil", False),
        # for whatever reason, "bpy.types.GreasePencilv3" is not set during introspection.
        # "grease_pencil": ("GreasePencilv3", False),
        "gpencil_data_owner": ("ID", False),
        # HairCurve/HairCurves are not defined in "bpy.types" during runtime (though there's a BlendDataHairCurves)
        # "curves": ("HairCurves", False),
        "id": ("ID", False),
        "image_paint_object": ("Object", False),
        "lattice": ("Lattice", False),
        "light": ("Light", False),
        "lightprobe": ("LightProbe", False),
        "line_style": ("FreestyleLineStyle", False),
        "material": ("Material", False),
        "material_slot": ("MaterialSlot", False),
        "mesh": ("Mesh", False),
        "meta_ball": ("MetaBall", False),
        "object": ("Object", False),
        "objects_in_mode": ("Object", True),
        "objects_in_mode_unique_data": ("Object", True),
        "particle_edit_object": ("Object", False),
        "particle_settings": ("ParticleSettings", False),
        "particle_system": ("ParticleSystem", False),
        "particle_system_editable": ("ParticleSystem", False),
        "property": ("ID", False),
        "pointcloud": ("PointCloud", False),
        # "pose_bone": ("PoseBone", False),
        "pose_object": ("Object", False),
        "scene": ("Scene", False),
        "sculpt_object": ("Object", False),
        "selectable_objects": ("Object", True),
        "selected_assets": ("AssetRepresentation", True),
        # "selected_bones": ("EditBone", True),
        "selected_editable_actions": ("Action", True),
        # "selected_editable_bones": ("EditBone", True),
        "selected_editable_fcurves": ("FCurve", True),
        "selected_editable_keyframes": ("Keyframe", True),
        "selected_editable_objects": ("Object", True),
        "selected_editable_sequences": ("Sequence", True),
        "selected_files": ("FileSelectEntry", True),
        "selected_ids": ("ID", True),
        "selected_nla_strips": ("NlaStrip", True),
        "selected_movieclip_tracks": ("MovieTrackingTrack", True),
        "selected_nodes": ("Node", True),
        "selected_objects": ("Object", True),
        # "selected_pose_bones": ("PoseBone", True),
        # "selected_pose_bones_from_active_object": ("PoseBone", True),
        "selected_sequences": ("Sequence", True),
        "selected_visible_actions": ("Action", True),
        "selected_visible_fcurves": ("FCurve", True),
        "sequences": ("Sequence", True),
        "soft_body": ("SoftBodyModifier", False),
        "speaker": ("Speaker", False),
        "texture": ("Texture", False),
        "texture_slot": ("TextureSlot", False),
        "texture_user": ("ID", False),
        "texture_user_property": ("Property", False),
        # "ui_list": ("UIList", False),
        "vertex_paint_object": ("Object", False),
        "view_layer": ("ViewLayer", False),
        # "visible_bones": ("EditBone", True),
        "visible_gpencil_layers": ("GPencilLayer", True),
        "visible_objects": ("Object", True),
        # "visible_pose_bones": ("PoseBone", True),
        "visible_fcurves": ("FCurve", True),
        "weight_paint_object": ("Object", False),
        "volume": ("Volume", False),
        "world": ("World", False),
    }

    for key, (target, is_collection) in items.items():
        output["properties"][key] = {
            ("collection" if is_collection else "pointer"): {
                "fixed_type": target,
                "identifier": key,
                "description": "",
                "type": "COLLECTION" if is_collection else "POINTER",
                "unit": "NONE",
                "subtype": "NONE",
                "is_required": False,
                "is_runtime": False,
                "is_output": False,
                "is_never_none": False,
            }
        }


def add_keyframe_methods(output):
    create_prop = lambda name, description, typ, required, **kwargs: {
        "identifier": name,
        "description": description,
        "type": typ,
        "unit": "NONE",
        "name": name,
        "subtype": "NONE",
        "is_argument_optional": not required,
        "is_required": required,
        "is_runtime": False,
        "is_output": False,
        "is_never_none": False,
        **kwargs,
    }

    output["methods"]["keyframe_insert"] = {
        "type": "rna",
        "item": {
            "description": "Insert a keyframe on the property given, adding fcurves and animation data when necessary.",
            "use_self": True,
            "use_self_type": False,
            "parameters": [
                {
                    "string": create_prop(
                        "data_path",
                        "path to the property to key, analogous to the fcurve’s data path.",
                        "STRING",
                        True,
                        **{"length_max": 0, "default": ""},
                    )
                },
                {
                    "int": create_prop(
                        "index",
                        "array index of the property to key. Defaults to -1 which will key all indices or a single channel if the property is not an array.",
                        "INT",
                        False,
                        **{
                            "hard_min": -2147483648,
                            "hard_max": 2147483647,
                            "soft_min": -2147483648,
                            "soft_max": 2147483647,
                            "default": -1,
                        },
                    )
                },
                {
                    "float": create_prop(
                        "frame",
                        "The frame on which the keyframe is inserted, defaulting to the current frame.",
                        "FLOAT",
                        False,
                        **{
                            "hard_min": -2147483648,
                            "hard_max": 2147483647,
                            "soft_min": -2147483648,
                            "soft_max": 2147483647,
                        },
                    )
                },
                {
                    "string": create_prop(
                        "group",
                        "The name of the group the F-Curve should be added to if it doesn’t exist yet.",
                        "STRING",
                        False,
                        **{"length_max": 0, "default": ""},
                    )
                },
                {
                    "string": create_prop(
                        "options",
                        "Optional set of flags",
                        "STRING",
                        False,
                        **{"length_max": 0, "default": ""},
                    )
                },
            ],
        },
    }


CLASSES = set()
EXTRAS = {
    bpy.types.bpy_struct: add_keyframe_methods,
    bpy.types.Context: add_context_properties,
    bpy.types.RenderEngine: disambiguate_render_property,
}


def inspect_recursive(tree, parent=object, output=[]):
    for cls, item in tree.items():
        CLASSES.add(cls.__name__)
        result = {"name": cls.__name__, "parent": parent.__name__}
        output.append(result)
        inspect_bpy_type(cls, result)

        if cls in EXTRAS:
            EXTRAS[cls](result)
        inspect_recursive(item, cls, output)

    return output


classes = inspect_recursive(root[object])


# fixup "bad" pointers: pointers and collections may refer to types
# outside of `bpy.types` (in particular, to objects in the Cycles
# renderer.)
for pointer in POINTERS:
    if pointer["fixed_type"] not in CLASSES:
        pointer["fixed_type"] = "bpy_struct"

operators = {}
for opmod_name in dir(bpy.ops):
    if "__" in opmod_name:
        continue

    operators[opmod_name] = {}
    opmod = getattr(bpy.ops, opmod_name)
    for operator_name in dir(opmod):
        if "__" in operator_name:
            continue
        operator = getattr(opmod, operator_name)
        rna_type = operator.get_rna_type()
        if rna_type is None:
            continue

        parameters = []
        for prop_name, descriptor in rna_type.properties.items():
            if prop_name == "rna_type":
                continue

            parameters.append(
                unpack_property_metadata(
                    descriptor
                )
            )

        operators[opmod_name][operator_name] = {
            "description": rna_type.description,
            "parameters": parameters,
        }

print(json.dumps({"classes": classes, "operators": operators}, indent=2))
