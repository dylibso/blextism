use serde::{ Serialize, Deserialize };
use std::collections::{HashMap, HashSet};
use smartstring::alias::String;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use heck::ToUpperCamelCase;

#[derive(Deserialize, Debug, Serialize)]
enum BpyType {
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "INT")]
    Int,
    #[serde(rename = "FLOAT")]
    Float,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "ENUM")]
    Enum,
    #[serde(rename = "POINTER")]
    Pointer,
    #[serde(rename = "COLLECTION")]
    Collection,
}

#[derive(Deserialize, Debug, Serialize)]
enum BpyUnit {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "LENGTH")]
    Length,
    #[serde(rename = "AREA")]
    Area,
    #[serde(rename = "VOLUME")]
    Volume,
    #[serde(rename = "ROTATION")]
    Rotation,
    #[serde(rename = "TIME")]
    Time,
    #[serde(rename = "TIME_ABSOLUTE")]
    TimeAbsolute,
    #[serde(rename = "VELOCITY")]
    Velocity,
    #[serde(rename = "ACCELERATION")]
    Acceleration,
    #[serde(rename = "MASS")]
    Mass,
    #[serde(rename = "CAMERA")]
    Camera,
    #[serde(rename = "POWER")]
    Power,
    #[serde(rename = "TEMPERATURE")]
    Temperature,
}

#[derive(Deserialize, Debug, Serialize)]
enum BpySubtype {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "FILE_PATH")]
    FilePath,
    #[serde(rename = "DIR_PATH")]
    DirPath,
    #[serde(rename = "FILE_NAME")]
    FileName,
    #[serde(rename = "BYTE_STRING")]
    ByteString,
    #[serde(rename = "PASSWORD")]
    Password,
    #[serde(rename = "PIXEL")]
    Pixel,
    #[serde(rename = "UNSIGNED")]
    Unsigned,
    #[serde(rename = "PERCENTAGE")]
    Percentage,
    #[serde(rename = "FACTOR")]
    Factor,
    #[serde(rename = "ANGLE")]
    Angle,
    #[serde(rename = "TIME")]
    Time,
    #[serde(rename = "TIME_ABSOLUTE")]
    TimeAbsolute,
    #[serde(rename = "DISTANCE")]
    Distance,
    #[serde(rename = "DISTANCE_CAMERA")]
    DistanceCamera,
    #[serde(rename = "POWER")]
    Power,
    #[serde(rename = "TEMPERATURE")]
    Temperature,
    #[serde(rename = "COLOR")]
    Color,
    #[serde(rename = "TRANSLATION")]
    Translation,
    #[serde(rename = "DIRECTION")]
    Direction,
    #[serde(rename = "VELOCITY")]
    Velocity,
    #[serde(rename = "ACCELERATION")]
    Acceleration,
    #[serde(rename = "MATRIX")]
    Matrix,
    #[serde(rename = "EULER")]
    Euler,
    #[serde(rename = "QUATERNION")]
    Quaternion,
    #[serde(rename = "AXISANGLE")]
    Axisangle,
    #[serde(rename = "XYZ")]
    Xyz,
    #[serde(rename = "XYZ_LENGTH")]
    XyzLength,
    #[serde(rename = "COLOR_GAMMA")]
    ColorGamma,
    #[serde(rename = "COORDINATES")]
    Coordinates,
    #[serde(rename = "LAYER")]
    Layer,
    #[serde(rename = "LAYER_MEMBER")]
    LayerMember,

    #[serde(rename = "")]
    Absent,
}

#[derive(Deserialize, Debug, Serialize)]
struct BpyPropertyItem {
    identifier: String,
    description: Option<String>,
    #[serde(rename = "type")]
    prop_type: BpyType,
    unit: BpyUnit,
    subtype: BpySubtype,
    is_required: bool,
    is_runtime: bool,
    is_output: bool,
    is_never_none: bool,
}

#[derive(Deserialize, Debug, Serialize)]
struct BpyPropertyArray<T> {
    default: Vec<T>,
    dimensions: [u32; 3],
    length: u32
}

#[derive(Deserialize, Debug, Serialize)]
struct BpyPropertyNumber<T> {
    hard_min: T,
    hard_max: T,
    soft_min: T,
    soft_max: T
}

#[derive(Deserialize, Debug, Serialize)]
struct BpyPropertyEnumItem {
    id: String,
    name: String,
    value: i32,
    description: String,
}

#[derive(Deserialize, Debug, Serialize)]
enum BpyProperty {
    #[serde(rename = "bool")]
    Boolean {
        #[serde(flatten)]
        item: BpyPropertyItem,
    },

    #[serde(rename = "bool[]")]
    BooleanArray {
        #[serde(flatten)]
        item: BpyPropertyItem,
        #[serde(flatten)]
        array: BpyPropertyArray<bool>,
    },

    #[serde(rename = "int")]
    Int {
        #[serde(flatten)]
        item: BpyPropertyItem,
        #[serde(flatten)]
        number: BpyPropertyNumber<i64>,
    },

    #[serde(rename = "int[]")]
    IntArray {
        #[serde(flatten)]
        item: BpyPropertyItem,
        #[serde(flatten)]
        array: BpyPropertyArray<i64>,
        #[serde(flatten)]
        number: BpyPropertyNumber<i64>,
    },

    #[serde(rename = "float")]
    Float {
        #[serde(flatten)]
        item: BpyPropertyItem,
        #[serde(flatten)]
        number: BpyPropertyNumber<f64>,
    },

    #[serde(rename = "float[]")]
    FloatArray {
        #[serde(flatten)]
        item: BpyPropertyItem,
        #[serde(flatten)]
        array: BpyPropertyArray<f32>,
        #[serde(flatten)]
        number: BpyPropertyNumber<f64>,
    },

    #[serde(rename = "string")]
    String {
        #[serde(flatten)]
        item: BpyPropertyItem,
        length_max: u32,
        default: Option<String>,
    },

    #[serde(rename = "enum")]
    Enum {
        #[serde(flatten)]
        item: BpyPropertyItem,
        items: Vec<BpyPropertyEnumItem>,
    },

    #[serde(rename = "pointer")]
    Pointer {
        #[serde(flatten)]
        item: BpyPropertyItem,
        fixed_type: String,
    },

    #[serde(rename = "collection")]
    Collection {
        #[serde(flatten)]
        item: BpyPropertyItem,
        fixed_type: String,
        collection: Option<String>,
    },
}

#[derive(Deserialize, Debug, Serialize)]
struct BpyMethod {
    #[serde(rename = "type")]
    kind: String
}

#[derive(Deserialize, Debug, Serialize)]
struct BpyStructure {
    name: String,
    parent: String,
    properties: HashMap<String, BpyProperty>,
    methods: HashMap<String, BpyMethod>,
}

fn property_codegen(properties: &HashMap<String, BpyProperty>, defined: &mut HashSet<std::string::String>) -> (TokenStream, TokenStream, TokenStream) {
    let mut impl_members: Vec<TokenStream> = Vec::with_capacity(16);
    let mut trait_members: Vec<TokenStream> = Vec::with_capacity(16);
    let mut extra_items: Vec<TokenStream> = Vec::with_capacity(16);

    for (func_name, property) in properties {
        let func_name = func_name.as_str();
        match property {
            BpyProperty::Boolean { item } => {
                let t = if item.is_never_none {
                    quote! { bool }
                } else {
                    quote! { Option<bool> }
                };
                let getter = format_ident!("get_{}", func_name);
                let setter = format_ident!("set_{}", func_name);
                let description = item.description.as_ref().map(|xs| xs.as_str());
                let description = if let Some(desc) = description {
                    quote! { #[doc = #desc] }
                } else {
                    quote! {}
                };

                trait_members.push(quote! {
                    #description
                    fn #getter(&self) -> #t;
                    fn #setter(&self, arg: #t);
                });


                // impl for BpyPtr
                impl_members.push(quote! {
                    fn #getter(&self) -> #t {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args);
                        let result: #t = serde_json::from_value(result).expect("TKTK(improve this msg): expected an boolean value");
                        result
                    }

                    fn #setter(&self, arg: #t) {
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            BpyProperty::BooleanArray { item, array } => {
                let t_in = if item.is_never_none {
                    quote! { &[bool] }
                } else {
                    quote! { Option<&[bool]> }
                };
                let t_out = if item.is_never_none {
                    quote! { Vec<bool> }
                } else {
                    quote! { Option<Vec<bool>> }
                };
                let getter = format_ident!("get_{}", func_name);
                let setter = format_ident!("set_{}", func_name);
                let description = item.description.as_ref().map(|xs| xs.as_str());
                let description = if let Some(desc) = description {
                    quote! { #[doc = #desc] }
                } else {
                    quote! {}
                };
                trait_members.push(quote! {
                    #description
                    fn #getter(&self) -> #t_out;
                    fn #setter(&self, arg: #t_in);
                });

                impl_members.push(quote! {
                    fn #getter(&self) -> #t_out {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args);
                        let result: #t_out = serde_json::from_value(result).expect("TKTK(improve this msg): expected an boolean value");
                        result
                    }

                    fn #setter(&self, arg: #t_in) {
                        // TKTK: TODO: extend arg if it's the wrong size.
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            BpyProperty::Int { item, number } => {
                let base_type = if number.hard_min == number.soft_min && number.soft_min == 0 {
                    quote! { u64 }
                } else {
                    quote! { i64 }
                };
                let t = if item.is_never_none {
                    quote! { #base_type }
                } else {
                    quote! { Option<#base_type> }
                };
                let getter = format_ident!("get_{}", func_name);
                let setter = format_ident!("set_{}", func_name);
                let description = item.description.as_ref().map(|xs| xs.as_str());
                let description = if let Some(desc) = description {
                    quote! { #[doc = #desc] }
                } else {
                    quote! {}
                };
                trait_members.push(quote! {
                    #description
                    fn #getter(&self) -> #t;
                    fn #setter(&self, arg: #t);
                });

                // impl for BpyPtr
                impl_members.push(quote! {
                    fn #getter(&self) -> #t {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args);
                        let result: #t = serde_json::from_value(result).expect("TKTK(improve this msg): expected an integer value");
                        result
                    }

                    fn #setter(&self, arg: #t) {
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            BpyProperty::IntArray { item, array, number } => {
                let base_type = if number.hard_min == number.soft_min && number.soft_min == 0 {
                    quote! { u64 }
                } else {
                    quote! { i64 }
                };

                let t_in = if item.is_never_none {
                    quote! { &[#base_type] }
                } else {
                    quote! { Option<&[#base_type]> }
                };
                let t_out = if item.is_never_none {
                    quote! { Vec<#base_type> }
                } else {
                    quote! { Option<Vec<#base_type>> }
                };
                let getter = format_ident!("get_{}", func_name);
                let setter = format_ident!("set_{}", func_name);
                let description = item.description.as_ref().map(|xs| xs.as_str());
                let description = if let Some(desc) = description {
                    quote! { #[doc = #desc] }
                } else {
                    quote! {}
                };
                trait_members.push(quote! {
                    #description
                    fn #getter(&self) -> #t_out;
                    fn #setter(&self, arg: #t_in);
                });

                // impl for BpyPtr
                impl_members.push(quote! {
                    fn #getter(&self) -> #t_out {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args);
                        let result: #t_out = serde_json::from_value(result).expect("TKTK(improve this msg): expected a list of integer values");
                        result
                    }

                    fn #setter(&self, arg: #t_in) {
                        // TKTK: TODO: extend arg if it's the wrong size.
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            BpyProperty::Float { item, number } => {
                let t = if item.is_never_none {
                    quote! { f64 }
                } else {
                    quote! { Option<f64> }
                };
                let getter = format_ident!("get_{}", func_name);
                let setter = format_ident!("set_{}", func_name);
                let description = item.description.as_ref().map(|xs| xs.as_str());
                let description = if let Some(desc) = description {
                    quote! { #[doc = #desc] }
                } else {
                    quote! {}
                };
                trait_members.push(quote! {
                    #description
                    fn #getter(&self) -> #t;
                    fn #setter(&self, arg: #t);
                });

                // impl for BpyPtr
                impl_members.push(quote! {
                    fn #getter(&self) -> #t {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args);
                        let result: #t = serde_json::from_value(result).expect("TKTK(improve this msg): expected an floating-point value");
                        result
                    }

                    fn #setter(&self, arg: #t) {
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            BpyProperty::FloatArray { item, array, number } => {
                let t_in = if item.is_never_none {
                    quote! { &[f64] }
                } else {
                    quote! { Option<&[f64]> }
                };
                let t_out = if item.is_never_none {
                    quote! { Vec<f64> }
                } else {
                    quote! { Option<Vec<f64>> }
                };
                let getter = format_ident!("get_{}", func_name);
                let setter = format_ident!("set_{}", func_name);
                let description = item.description.as_ref().map(|xs| xs.as_str());
                let description = if let Some(desc) = description {
                    quote! { #[doc = #desc] }
                } else {
                    quote! {}
                };
                trait_members.push(quote! {
                    #description
                    fn #getter(&self) -> #t_out;
                    fn #setter(&self, arg: #t_in);
                });

                // impl for BpyPtr
                impl_members.push(quote! {
                    fn #getter(&self) -> #t_out {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args);
                        let result: #t_out = serde_json::from_value(result).expect("TKTK(improve this msg): expected a list of floating-point values");
                        result
                    }

                    fn #setter(&self, arg: #t_in) {
                        // TKTK: TODO: extend arg if it's the wrong size.
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            BpyProperty::String { item, length_max, default } => {
                let t_in = if item.is_never_none {
                    quote! { &str }
                } else {
                    quote! { Option<&str> }
                };
                let t_out = if item.is_never_none {
                    quote! { String }
                } else {
                    quote! { Option<String> }
                };
                let getter = format_ident!("get_{}", func_name);
                let setter = format_ident!("set_{}", func_name);
                let description = item.description.as_ref().map(|xs| xs.as_str());
                let description = if let Some(desc) = description {
                    quote! { #[doc = #desc] }
                } else {
                    quote! {}
                };
                trait_members.push(quote! {
                    #description
                    fn #getter(&self) -> #t_out;
                    fn #setter(&self, arg: #t_in);
                });

                // impl for BpyPtr
                impl_members.push(quote! {
                    fn #getter(&self) -> #t_out {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args);
                        let result: #t_out = serde_json::from_value(result).expect("TKTK(improve this msg): expected a string");
                        result
                    }

                    fn #setter(&self, arg: #t_in) {
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            BpyProperty::Enum { item, items } => {
                // TODO!
            },

            BpyProperty::Pointer { item, fixed_type } => {
                let t = if item.is_never_none {
                    quote! { BpyPtr }
                } else {
                    quote! { Option<BpyPtr> }
                };


                let target_type = format_ident!("{}", fixed_type.as_str().to_upper_camel_case());
                let out_type = if item.is_never_none {
                    quote! { Box<dyn #target_type + Send + Sync> }
                } else {
                    quote! { Option<Box<dyn #target_type + Send + Sync>> }
                };

                let getter = format_ident!("get_{}", func_name);
                let setter = format_ident!("set_{}", func_name);
                let description = item.description.as_ref().map(|xs| xs.as_str());
                let description = if let Some(desc) = description {
                    quote! { #[doc = #desc] }
                } else {
                    quote! {}
                };
                trait_members.push(quote! {
                    #description
                    fn #getter(&self) -> #out_type;
                    fn #setter(&self, arg: #t);
                });

                let unwrap = if item.is_never_none {
                    quote! {
                        let result: BpyPtr = serde_json::from_value(result).expect("TKTK(improve this msg): expected an floating-point value");
                        let result = Box::new(result) as Box<dyn #target_type + Send + Sync>;
                    }
                } else {
                    quote! {
                        let result: Option<BpyPtr> = serde_json::from_value(result).expect("TKTK(improve this msg): expected an floating-point value");

                        let result = match result {
                            Some(xs) => Some(Box::new(xs) as Box<dyn #target_type + Send + Sync>),
                            None => None,
                        };
                    }
                };

                // impl for BpyPtr
                impl_members.push(quote! {
                    fn #getter(&self) -> #out_type {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args);
                        #unwrap
                        result
                    }

                    fn #setter(&self, arg: #t) {
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            // TODO: BpyProperty::Collection isn't JUST a vec
            // it should return a Box<dyn #collection>; though
            // it might need to "mix in" a `bpy_prop_collection` type?
            BpyProperty::Collection { item, fixed_type, collection } => {
                let collection_constraint = collection.as_ref().map(|c| {
                    let ident = format_ident!("{}", c.as_str().to_upper_camel_case());
                    quote! { : #ident + BpyPropCollection }
                }).unwrap_or_else(|| {
                    quote! { : BpyStruct + BpyPropCollection }
                });

                let target_type = format_ident!("{}", fixed_type.as_str().to_upper_camel_case());
                let return_type_str = format!("BpyPropCollection_{}", collection.as_ref().unwrap_or(fixed_type)).to_upper_camel_case();
                let return_type_ident = format_ident!("{}", return_type_str);

                // TODO: how to push the target_type back up?
                if !defined.contains(&return_type_str) {
                    defined.insert(return_type_str);
                    extra_items.push(quote! {
                        pub trait #return_type_ident #collection_constraint {}

                        impl #return_type_ident for BpyPtr {}
                    });
                }

                let out_type = if item.is_never_none {
                    quote! { Box<dyn #return_type_ident + Send + Sync> }
                } else {
                    quote! { Option<Box<dyn #return_type_ident + Send + Sync>> }
                };

                let getter = format_ident!("get_{}", func_name);
                let description = item.description.as_ref().map(|xs| xs.as_str());
                let description = if let Some(desc) = description {
                    quote! { #[doc = #desc] }
                } else {
                    quote! {}
                };
                trait_members.push(quote! {
                    #description
                    fn #getter(&self) -> #out_type;
                });

                let unwrap = if item.is_never_none {
                    quote! {
                        let result: BpyPtr = serde_json::from_value(result).expect("TKTK(improve this msg): expected an floating-point value");
                        let result = Box::new(result) as Box<dyn #return_type_ident + Send + Sync>;
                    }
                } else {
                    quote! {
                        let result: Option<BpyPtr> = serde_json::from_value(result).expect("TKTK(improve this msg): expected an floating-point value");

                        let result = match result {
                            Some(xs) => Some(Box::new(xs) as Box<dyn #return_type_ident + Send + Sync>),
                            None => None,
                        };
                    }
                };
                // impl for BpyPtr
                impl_members.push(quote! {
                    fn #getter(&self) -> #out_type {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args);
                        #unwrap
                        result
                    }
                });
            },

            _ => {}
        }
    }

    (extra_items.into_iter().collect(), trait_members.into_iter().collect(), impl_members.into_iter().collect())
}

fn structure_to_syntax(structure: BpyStructure, defined: &mut HashSet<std::string::String>) -> TokenStream {
    if structure.name == "type" {
        return quote!{}
    }

    let is_top = structure.parent == "object" || structure.parent == "type";
    let name = format_ident!("{}", structure.name.as_str().to_upper_camel_case());
    let parent = format_ident!("{}", structure.parent.as_str().to_upper_camel_case());

    let (extra_items, mut trait_members, mut impl_members) = property_codegen(&structure.properties, defined);

    #[allow(clippy::single_match)]
    match structure.name.as_str() {
        "bpy_prop_collection" => {
            trait_members.extend(quote! {
                fn get(&self, key: &str) -> Option<BpyPtr>;
            });

            impl_members.extend(quote! {
                fn get(&self, key: &str) -> Option<BpyPtr> {
                    let args = PyArgs::args(self, key);
                    let result = invoke_bpy_callmethod("get", args);
                    let result: Option<BpyPtr> = serde_json::from_value(result).expect("TKTK");
                    result
                }
            });
        },

        _ => {}
    }

    let parent = if !is_top {
        quote! { : #parent }
    } else {
        quote! { }
    };

    quote! {
        #extra_items

        pub trait #name #parent {
            #trait_members
        }

        impl #name for BpyPtr {
            #impl_members
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input = std::env::args().nth(1).unwrap_or_else(|| {
        "/dev/stdin".to_string()
    });
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(input.as_str())?;

    let mut defined = HashSet::new();
    let structures: Vec<BpyStructure> = serde_json::from_reader(std::io::BufReader::new(file))?;
    let results: Vec<_> = structures.into_iter().rev().map(|xs| structure_to_syntax(xs, &mut defined)).collect();

    let results: TokenStream = results.into_iter().collect();

    let data_targets = [
        ("actions", "BlendDataActions"),
        ("armatures", "BlendDataArmatures"),
        ("bl_rna", "Struct"),
        ("brushes", "BlendDataBrushes"),
        ("cache_files", "BlendDataCacheFiles"),
        ("cameras", "BlendDataCameras"),
        ("collections", "BlendDataCollections"),
        ("curves", "BlendDataCurves"),
        ("fonts", "BlendDataFonts"),
        ("grease_pencils", "BlendDataGreasePencils"),
        ("hair_curves", "BlendDataHairCurves"),
        ("images", "BlendDataImages"),
        ("lattices", "BlendDataLattices"),
        ("libraries", "BlendDataLibraries"),
        ("lightprobes", "BlendDataProbes"),
        ("lights", "BlendDataLights"),
        ("linestyles", "BlendDataLineStyles"),
        ("masks", "BlendDataMasks"),
        ("materials", "BlendDataMaterials"),
        ("meshes", "BlendDataMeshes"),
        ("metaballs", "BlendDataMetaBalls"),
        ("movieclips", "BlendDataMovieClips"),
        ("node_groups", "BlendDataNodeTrees"),
        ("objects", "BlendDataObjects"),
        ("paint_curves", "BlendDataPaintCurves"),
        ("palettes", "BlendDataPalettes"),
        ("particles", "BlendDataParticles"),
        ("pointclouds", "BlendDataPointClouds"),
        ("rna_type", "Struct"),
        ("scenes", "BlendDataScenes"),
        ("screens", "BlendDataScreens"),
        ("sounds", "BlendDataSounds"),
        ("speakers", "BlendDataSpeakers"),
        ("texts", "BlendDataTexts"),
        ("textures", "BlendDataTextures"),
        ("volumes", "BlendDataVolumes"),
        ("window_managers", "BlendDataWindowManagers"),
        ("workspaces", "BlendDataWorkSpaces"),
        ("worlds", "BlendDataWorlds")
    ];

    let mut bpy_data_items: Vec<_> = Vec::with_capacity(data_targets.len());
    let mut bpy_data_impls: Vec<_> = Vec::with_capacity(data_targets.len());
    for (func, target) in data_targets {
        let func = format_ident!("{}", func);
        let target = format_ident!("{}", format!("BpyPropCollection_{}", target).to_upper_camel_case());
        bpy_data_items.push(quote! {
            #func: i64,
        });
        bpy_data_impls.push(quote! {
            pub fn #func() -> Box<dyn super::types::#target + Send + Sync> {
                Box::new(BpyPtr { ptr: load_bpy_data().#func })
            }
        });
    }

    let bpy_data_items: TokenStream = bpy_data_items.into_iter().collect();
    let bpy_data_impls: TokenStream = bpy_data_impls.into_iter().collect();

    let module = quote! {
        pub(crate) mod bpy {
            use serde::{ Deserialize, Serialize };
            use smartstring::alias::String;
            use crate::{ invoke_bpy_setattr, invoke_bpy_getattr, invoke_bpy_callmethod };
            use std::collections::HashMap;

            #[derive(Serialize, Deserialize, Clone, Debug)]
            pub struct BpyPtr {
                #[serde(rename = "@ptr")]
                ptr: i64,
            }

            #[derive(Serialize, Deserialize, Default)]
            pub struct PyArgs {
                #[serde(rename = "self")]
                target: Option<BpyPtr>,
                args: Option<Vec<serde_json::Value>>,
                kwargs: Option<HashMap<String, serde_json::Value>>,
            }

            impl PyArgs {
                fn new(target: &BpyPtr) -> Self {
                    Self {
                        target: Some(target.clone()),
                        ..Default::default()
                    }
                }

                fn args(target: &BpyPtr, args: impl Serialize) -> Self {
                    let value = serde_json::to_value(args).expect("pyarg must be serializable");
                    Self {
                        target: Some(target.clone()),
                        args: Some(vec![value]),
                        ..Default::default()
                    }
                }
            }

            pub mod types {
                use super::*;
                #results
            }

            pub mod data {
                use super::*;
                use extism_pdk;

                #[derive(Deserialize)]
                struct BpyData {
                    #bpy_data_items
                }

                static mut BPY_DATA: Option<BpyData> = None;

                fn load_bpy_data() -> &'static BpyData {
                    if let Some(data) = unsafe { BPY_DATA.as_ref() } {
                        return data;
                    }

                    let cfg = extism_pdk::config::get("bpy.data");
                    let data: BpyData = serde_json::from_str(
                           cfg 
                                .expect("'bpy.data' extism config must be set")
                                .expect("'bpy.data' should be set")
                                .as_str(),
                        )
                        .expect("'bpy.data' must contain valid JSON");

                    unsafe { BPY_DATA = Some(data) };
                    load_bpy_data()
                }

                #bpy_data_impls
            }
        }
    };

    let syntree = syn::parse2(module)?;
    println!("{}", prettyplease::unparse(&syntree));


    Ok(())
}
