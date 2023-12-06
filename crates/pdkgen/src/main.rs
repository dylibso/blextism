use serde::{ Serialize, Deserialize };
use std::{collections::HashMap, io::Read};
use smartstring::alias::String;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

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

fn property_codegen(properties: &HashMap<String, BpyProperty>) -> (TokenStream, TokenStream) {
    let mut impl_members: Vec<TokenStream> = Vec::with_capacity(16);
    let mut trait_members: Vec<TokenStream> = Vec::with_capacity(16);
    let mut enums: Vec<()> = Vec::with_capacity(16);

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

                        let result = invoke_bpy_getattr(#func_name, args).into_inner();
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

                        let result = invoke_bpy_getattr(#func_name, args).into_inner();
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

                        let result = invoke_bpy_getattr(#func_name, args).into_inner();
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

                        let result = invoke_bpy_getattr(#func_name, args).into_inner();
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

                        let result = invoke_bpy_getattr(#func_name, args).into_inner();
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

                        let result = invoke_bpy_getattr(#func_name, args).into_inner();
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

                        let result = invoke_bpy_getattr(#func_name, args).into_inner();
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


                let out_type = format_ident!("{}", fixed_type.as_str());
                let out_type = if item.is_never_none {
                    quote! { Box<dyn #out_type + Send + Sync> }
                } else {
                    quote! { Option<Box<dyn #out_type + Send + Sync>> }
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
                    fn #getter(&self) -> #out_type {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args).into_inner();
                        let result: #t = serde_json::from_value(result).expect("TKTK(improve this msg): expected an floating-point value");

                        result as #out_type
                    }

                    fn #setter(&self, arg: #t) {
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            BpyProperty::Collection { item, fixed_type } => {
                let t = if item.is_never_none {
                    quote! { BpyPtr }
                } else {
                    quote! { Option<BpyPtr> }
                };


                let out_type = format_ident!("{}", fixed_type.as_str());
                let out_type = if item.is_never_none {
                    quote! { Box<dyn #out_type + Send + Sync> }
                } else {
                    quote! { Option<Box<dyn #out_type + Send + Sync>> }
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
                    fn #getter(&self) -> #out_type {
                        let args = PyArgs::new(self);

                        let result = invoke_bpy_getattr(#func_name, args).into_inner();
                        let result: #t = serde_json::from_value(result).expect("TKTK(improve this msg): expected an floating-point value");

                        result as #out_type
                    }

                    fn #setter(&self, arg: #t) {
                        let args = PyArgs::args(self, arg);

                        invoke_bpy_setattr(#func_name, args);
                    }
                });
            },

            _ => {}
        }
    }

    (trait_members.into_iter().collect(), impl_members.into_iter().collect())
}

fn structure_to_syntax(structure: BpyStructure) -> TokenStream {
    if structure.name == "type" {
        return quote!{}
    }

    let is_top = structure.parent == "object" || structure.parent == "type";
    eprintln!("is_top={}; name={}; parent={}", is_top, structure.name, structure.parent);
    let name = format_ident!("{}", structure.name.as_str());
    let parent = format_ident!("{}", structure.parent.as_str());

    let (trait_members, impl_members) = property_codegen(&structure.properties);

    let parent = if !is_top {
        quote! { : #parent }
    } else {
        quote! { }
    };

    quote! {
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
    eprintln!("input={:?}", input);
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(input.as_str())?;

    let structures: Vec<BpyStructure> = serde_json::from_reader(std::io::BufReader::new(file))?;
    let results: Vec<_> = structures.into_iter().rev().map(structure_to_syntax).collect();

    let results: TokenStream = results.into_iter().collect();

    let module = quote! {
        mod bpy {
            use serde::{ Deserialize, Serialize };
            use smartstring::alias::String;

            #[derive(Serialize, Deserialize, Clone)]
            pub struct BpyPtr {
                #[serde(rename = "@ptr")]
                ptr: usize,
                #[serde(rename = "@cls")]
                cls: String,
            }

            #[derive(Serialize, Deserialize, Default)]
            pub struct PyArgs {
                #[serde(rename = "self")]
                target: Option<BpyPtr>,
                args: Option<Vec<serde_json::Value>>,
                kwargs: Option<serde_json::Map<String, serde_json::Value>>
            }

            impl PyArgs {
                fn new(target: &BpyPtr) {
                    Self {
                        target: Some(target.clone()),
                        ..Default::default()
                    }
                }

                fn args(target: &BpyPtr, args: impl Serialize) {
                    let value = serde_json::to_value(args).expect("pyarg must be serializable");
                    Self {
                        target: Some(target.clone()),
                        args: vec![value],
                        ..Default::default()
                    }
                }
            }

            #results
        }
    };

    let syntree = syn::parse2(module)?;
    println!("{}", prettyplease::unparse(&syntree));


    Ok(())
}
