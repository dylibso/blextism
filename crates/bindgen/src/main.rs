use serde::{ Serialize, Deserialize };
use std::collections::{HashMap, HashSet};
use smartstring::alias::String;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use heck::{ToSnekCase, ToUpperCamelCase};

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
    name: Option<String>,
    description: Option<String>,
    #[serde(rename = "type")]
    prop_type: BpyType,
    unit: BpyUnit,
    subtype: BpySubtype,
    is_required: bool,
    #[serde(default)]
    is_argument_optional: bool,
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

fn safe_ident(inp: &str) -> proc_macro2::Ident {
    let as_str =  match inp {
        "fn" => "r#fn",
        "use" => "r#use",
        "move" => "r#move",
        "struct" => "r#struct",
        "enum" => "r#enum",
        "self" => "_self",
        "union" => "r#union",
        "trait" => "r#trait",
        "type" => "r#type",
        "match" => "r#match",
        "box" => "r#box",
        "impl" => "r#impl",
        xs => xs
    };
    format_ident!("{}", as_str)
}

impl BpyProperty {
    fn as_item(&self) -> &BpyPropertyItem {
        match self {
            BpyProperty::Boolean { item } => item,
            BpyProperty::BooleanArray { item, .. } => item,
            BpyProperty::Int { item, .. } => item,
            BpyProperty::IntArray { item, .. } => item,
            BpyProperty::Float { item, .. } => item,
            BpyProperty::FloatArray { item, .. } => item,
            BpyProperty::String { item, .. } => item,
            BpyProperty::Enum { item, .. } => item,
            BpyProperty::Pointer { item, .. } => item,
            BpyProperty::Collection { item, .. } => item,
        }
    }

    fn is_output(&self) -> bool {
        self.as_item().is_output
    }

    fn as_setter_attr_name(&self) -> proc_macro2::Ident {
        return format_ident!("set_{}", self.as_item().identifier.as_str());
    }

    fn as_getter_attr_name(&self) -> proc_macro2::Ident {
        safe_ident(self.as_item().identifier.as_str())
    }

    fn as_method_parameter(&self, extra_items: &mut Vec<TokenStream>) -> TokenStream {
        let tk = match self {
            BpyProperty::Boolean { item } => quote! { bool },
            BpyProperty::BooleanArray { item, array } => quote! { &[bool] },
            BpyProperty::Int { item, number } => if number.hard_min == number.soft_min && number.soft_min == 0 {
                quote! { u64 }
            } else {
                quote! { i64 }
            },
            BpyProperty::IntArray { item, array, number } => if number.hard_min == number.soft_min && number.soft_min == 0 {
                quote! { Vec<u64> }
            } else {
                quote! { Vec<i64> }
            },
            BpyProperty::Float { item, number } => quote! { f64 },
            BpyProperty::FloatArray { item, array, number } => quote! { &[f64] },
            BpyProperty::String { item, length_max, default } => quote! { &str },
            BpyProperty::Enum { item, items } => {
                // for now, enums are strings.
                quote! { &str }
            },

            BpyProperty::Pointer { item, fixed_type } => {
                quote! { BpyPtr }
            },
            BpyProperty::Collection { item, fixed_type, collection } => {
                quote! { BpyPtr }
            },
        };

        let argname = self.as_item().identifier.as_str().to_snek_case();
        let argname = safe_ident(argname.as_str());

        if self.as_item().is_required {
            quote! { #argname: #tk }
        } else {
            quote! { #argname: Option<#tk> }
        }
    }

    fn as_setter_parameter_type(&self, extra_items: &mut Vec<TokenStream>) -> TokenStream {
        let tk = match self {
            BpyProperty::Boolean { item } => quote! { bool },
            BpyProperty::BooleanArray { item, array } => quote! { &[bool] },
            BpyProperty::Int { item, number } => if number.hard_min == number.soft_min && number.soft_min == 0 {
                quote! { u64 }
            } else {
                quote! { i64 }
            },
            BpyProperty::IntArray { item, array, number } => if number.hard_min == number.soft_min && number.soft_min == 0 {
                quote! { Vec<u64> }
            } else {
                quote! { Vec<i64> }
            },
            BpyProperty::Float { item, number } => quote! { f64 },
            BpyProperty::FloatArray { item, array, number } => quote! { &[f64] },
            BpyProperty::String { item, length_max, default } => quote! { &str },
            BpyProperty::Enum { item, items } => {
                // for now, enums are strings.
                quote! { &str }
            },

            BpyProperty::Pointer { item, fixed_type } => {
                quote! { BpyPtr }
            },
            BpyProperty::Collection { item, fixed_type, collection } => {
                quote! { BpyPtr }
            },
        };

        if self.as_item().is_required {
            tk
        } else {
            quote! { Option<#tk> }
        }
    }

    fn as_return_type(&self, extra_items: &mut Vec<TokenStream>, defined: &mut HashSet<std::string::String>) -> TokenStream {
        let tk = match self {
            BpyProperty::Boolean { item } => quote! { bool },
            BpyProperty::BooleanArray { item, array } => quote! { Vec<bool> },
            BpyProperty::Int { item, number } => if number.hard_min == number.soft_min && number.soft_min == 0 {
                quote! { u64 }
            } else {
                quote! { i64 }
            },
            BpyProperty::IntArray { item, array, number } => if number.hard_min == number.soft_min && number.soft_min == 0 {
                quote! { Vec<u64> }
            } else {
                quote! { Vec<i64> }
            },
            BpyProperty::Float { item, number } => quote! { f64 },
            BpyProperty::FloatArray { item, array, number } => quote! { Vec<f64> },
            BpyProperty::String { item, length_max, default } => quote! { String },
            BpyProperty::Enum { item, items } => {
                // for now, enums are strings.
                quote! { String }
            },

            BpyProperty::Pointer { item, fixed_type } => {
                let ident = format_ident!("{}", fixed_type.as_str().to_upper_camel_case());
                quote! { Box<dyn #ident + Send + Sync> }
            },
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
                        pub trait #return_type_ident #collection_constraint {
                            fn get(&self, key: &str) -> Option<Box<dyn #target_type + Send + Sync>>;
                            fn keys(&self) -> Vec<String>;
                            fn values(&self) -> Vec<Box<dyn #target_type + Send + Sync>>;
                            fn items(&self) -> Vec<(String, Box<dyn #target_type + Send + Sync>)>;
                        }

                        impl #return_type_ident for BpyPtr {
                            fn get(&self, key: &str) -> Option<Box<dyn #target_type + Send + Sync>> {
                                get(self, key)
                                    .map(Box::new)
                                    .map(|bx| bx as Box<dyn #target_type + Send + Sync>)
                            }
                            fn keys(&self) -> Vec<String> {
                                keys(self)
                            }

                            fn values(&self) -> Vec<Box<dyn #target_type + Send + Sync>> {
                                values(self).into_iter().map(|xs| Box::new(xs) as Box<dyn #target_type + Send + Sync>).collect()
                            }

                            fn items(&self) -> Vec<(String, Box<dyn #target_type + Send + Sync>)> {
                                items(self).into_iter().map(|(k, xs)| (k, Box::new(xs) as Box<dyn #target_type + Send + Sync>)).collect()
                            }
                        }
                    });
                }

                quote! { Box<dyn #return_type_ident + Send + Sync> }
            },
        };

        if self.as_item().is_never_none {
            tk
        } else {
            quote! { Option<#tk> }
        }
    }

    fn as_parsed_intermediate_value(&self) -> TokenStream {
        match self {
            BpyProperty::Boolean { .. } |
            BpyProperty::Int { .. } |
            BpyProperty::Float { .. } |
            BpyProperty::String { .. } |
            BpyProperty::BooleanArray { .. } |
            BpyProperty::IntArray { .. } |
            BpyProperty::FloatArray { .. } => {
                quote! {
                    serde_json::from_value(bpy_output).expect("expected to deserialize appropriately")
                }
            },
            BpyProperty::Pointer { item, fixed_type } => {
                let target_type = format_ident!("{}", fixed_type.as_str().to_upper_camel_case());
                if item.is_never_none {
                    quote! {
                        let result: BpyPtr = serde_json::from_value(bpy_output).expect("TKTK(improve this msg): expected an floating-point value");
                        Box::new(result) as Box<dyn #target_type + Send + Sync>
                    }
                } else {
                    quote! {
                        let result: Option<BpyPtr> = serde_json::from_value(bpy_output).expect("TKTK(improve this msg): expected an floating-point value");
                        match result {
                            Some(xs) => Some(Box::new(xs) as Box<dyn #target_type + Send + Sync>),
                            None => None,
                        }
                    }
                }
            },

            BpyProperty::Collection { item, fixed_type, collection } => {
                let target_type = format_ident!("{}", fixed_type.as_str().to_upper_camel_case());
                let return_type_str = format!("BpyPropCollection_{}", collection.as_ref().unwrap_or(fixed_type)).to_upper_camel_case();
                let return_type_ident = format_ident!("{}", return_type_str);

                if item.is_never_none {
                    quote! {
                        let result: BpyPtr = serde_json::from_value(bpy_output).expect("TKTK(improve this msg): expected an floating-point value");
                        Box::new(result) as Box<dyn #return_type_ident + Send + Sync>
                    }
                } else {
                    quote! {
                        let result: Option<BpyPtr> = serde_json::from_value(bpy_output).expect("TKTK(improve this msg): expected an floating-point value");

                        match result {
                            Some(xs) => Some(Box::new(xs) as Box<dyn #return_type_ident + Send + Sync>),
                            None => None,
                        }
                    }
                }
            },

            BpyProperty::Enum { item, items } => {
                quote! { serde_json::from_value(bpy_output).expect("expected to deserialize appropriately") }
            },
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type", content = "item")]
enum BpyMethod {
    #[serde(rename = "rna")]
    Rna {
        description: String,
        use_self: bool,
        use_self_type: bool,
        parameters: Vec<BpyProperty>
    },

    #[serde(rename = "builtin_function_or_method")]
    Builtin,

    #[serde(rename = "method_descriptor")]
    MethodDescriptor,
    #[serde(rename = "_PropertyDeferred")]
    PropertyDeferred,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "method")]
    Method,
}

#[derive(Deserialize, Debug, Serialize)]
struct BpyStructure {
    name: String,
    parent: String,
    properties: HashMap<String, BpyProperty>,
    methods: HashMap<String, BpyMethod>,
}

fn method_codegen(methods: &HashMap<String, BpyMethod>, defined: &mut HashSet<std::string::String>) -> (TokenStream, TokenStream, TokenStream) {

    let mut impl_members: Vec<TokenStream> = Vec::with_capacity(16);
    let mut trait_members: Vec<TokenStream> = Vec::with_capacity(16);
    let mut extra_items: Vec<TokenStream> = Vec::with_capacity(16);

    for (func_name, method) in methods {
        match method {
            BpyMethod::Rna { description, use_self, use_self_type, parameters } => {
                let func_name = func_name.as_str().to_snek_case();
                let func_name_ident = safe_ident(func_name.as_str());

                let description = description.as_str();
                let (outputs, inputs): (Vec<_>, Vec<_>) = parameters.iter().partition(|xs| {
                    xs.is_output()
                });

                let params: TokenStream = inputs.iter()
                    .map(|prop| prop.as_method_parameter(&mut extra_items))
                    .fold(TokenStream::new(), |stream, tk| { quote! { #stream, #tk } });

                let into_pyargs: TokenStream = inputs.iter()
                    .map(|prop| {
                        let prop = prop.as_item().identifier.as_str().to_snek_case();
                        safe_ident(prop.as_str())
                    })
                    .fold(TokenStream::new(), |stream, tk| { quote! { #stream serde_json::to_value(#tk).expect("pyarg must be serializable"), } });

                if outputs.len() > 1 {
                    continue
                }

                let return_type = if outputs.is_empty() {
                    quote! { }
                } else {
                    let output = outputs[0].as_return_type(&mut extra_items, defined);
                    quote! {
                        -> #output
                    }
                };

                let (assign_to, from_serde_value) = if outputs.is_empty() {
                    (quote! { }, quote! { })
                } else {
                    (quote! { let bpy_output = }, outputs[0].as_parsed_intermediate_value())
                };

                trait_members.push(quote! {
                    #[doc = #description]
                    fn #func_name_ident(&self #params) #return_type;
                });

                impl_members.push(quote! {
                    fn #func_name_ident(&self #params) #return_type {
                        let bpy_input = PyArgs::argv(self, vec![#into_pyargs]);
                        #assign_to invoke_bpy_callmethod(#func_name, bpy_input);
                        #from_serde_value
                    }
                });
            },
            BpyMethod::Builtin => {},
            BpyMethod::MethodDescriptor => {},
            BpyMethod::PropertyDeferred => {},
            BpyMethod::Function => {},
            BpyMethod::Method => {},
        }
    }

    (extra_items.into_iter().collect(), trait_members.into_iter().collect(), impl_members.into_iter().collect())
}

fn property_codegen(properties: &HashMap<String, BpyProperty>, defined: &mut HashSet<std::string::String>) -> (TokenStream, TokenStream, TokenStream) {
    let mut impl_members: Vec<TokenStream> = Vec::with_capacity(16);
    let mut trait_members: Vec<TokenStream> = Vec::with_capacity(16);
    let mut extra_items: Vec<TokenStream> = Vec::with_capacity(16);

    for (func_name, property) in properties {
        let func_name = func_name.as_str();

        let getter = property.as_getter_attr_name();
        let setter = property.as_setter_attr_name();
        let setter_param = property.as_setter_parameter_type(&mut extra_items);
        let return_type = property.as_return_type(&mut extra_items, defined);

        let description = property.as_item().description.as_ref().map(|xs| xs.as_str());
        let description = if let Some(desc) = description {
            quote! { #[doc = #desc] }
        } else {
            quote! {}
        };

        let parser = property.as_parsed_intermediate_value();

        trait_members.push(quote! {
            #description
            fn #getter(&self) -> #return_type;
            fn #setter(&self, arg: #setter_param);
        });

        // impl for BpyPtr
        impl_members.push(quote! {
            fn #getter(&self) -> #return_type {
                let args = PyArgs::new(self);

                let bpy_output = invoke_bpy_getattr(#func_name, args);
                #parser
            }

            fn #setter(&self, arg: #setter_param) {
                let args = PyArgs::arg1(self, arg);

                invoke_bpy_setattr(#func_name, args);
            }
        });
    }

    (extra_items.into_iter().collect(), trait_members.into_iter().collect(), impl_members.into_iter().collect())
}

fn structure_to_syntax(structure: BpyStructure, defined: &mut HashSet<std::string::String>) -> TokenStream {
    if structure.name == "type" {
        return quote!{}
    }

    let is_top = structure.parent == "object" || structure.parent == "type";
    let structure_name = structure.name.as_str().to_upper_camel_case();
    let name = format_ident!("{}", structure_name);
    let parent = format_ident!("{}", structure.parent.as_str().to_upper_camel_case());

    let (mut extra_items, mut trait_members, mut impl_members) = property_codegen(&structure.properties, defined);
    let (e, t, i) = method_codegen(&structure.methods, defined);

    extra_items.extend(e);
    trait_members.extend(t);
    impl_members.extend(i);

    if structure_name.as_str() == "BpyStruct" {
        trait_members.extend(quote! {
            /// Unbox a dynamic pointer. Useful for re-casting to a different trait object.
            fn to_bpy_ptr(&self) -> BpyPtr;
        });

        impl_members.extend(quote! {
            fn to_bpy_ptr(&self) -> BpyPtr {
                self.clone()
            }
        });
    }

    let parent = if !is_top {
        quote! { : #parent }
    } else {
        quote! { : std::fmt::Debug + private::Sealed }
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

#[derive(Deserialize, Debug, Serialize)]
struct BpyOperator {
    description: String,
    parameters: Vec<BpyProperty>
}

#[derive(Deserialize, Debug, Serialize)]
struct Schema {
    classes: Vec<BpyStructure>,
    operators: HashMap<String, HashMap<String, BpyOperator>>
}

fn ops_codegen(ops: HashMap<String, HashMap<String, BpyOperator>>) -> TokenStream {
    let mut tkstream = TokenStream::new();
    let extra_items: Vec<TokenStream> = Vec::with_capacity(16);
    for (mod_name, items) in ops.into_iter() {
        let mod_name_str = mod_name.as_str().to_snek_case();
        let mod_name_ident = safe_ident(mod_name_str.as_str());

        let mut ops = TokenStream::new();
        for (op_name, descriptor) in items.into_iter() {
            let op_name_str = op_name.as_str().to_snek_case();
            let op_name_ident = safe_ident(op_name_str.as_str());
            let inputs: Vec<_> = descriptor.parameters.iter().filter(|xs| {
                !xs.is_output()
            }).collect();

            let params_docs = inputs.iter()
                .map(|prop| {
                    let item = prop.as_item();
                    format!("- {}: {}", item.identifier, item.description.clone().unwrap_or_default())
                })
                .reduce(|lhs, rhs| format!("{}\n{}", lhs, rhs))
                .unwrap_or_default();

            let description = format!("{}\n{}", descriptor.description, params_docs);

            ops.extend(quote! {
                #[doc = #description]
                pub fn #op_name_ident (params: impl Into<PyArgs>) -> serde_json::Value {
                    invoke_bpy_operator(#mod_name_str, #op_name_str, params.into())
                }
            })

        }

        tkstream.extend(quote! {
            pub mod #mod_name_ident {
                use super::*;
                #ops
            }
        });
    }
    tkstream
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input = std::env::args().nth(1).unwrap_or_else(|| {
        "/dev/stdin".to_string()
    });
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(input.as_str())?;

    let mut defined = HashSet::new();
    let Schema { classes, operators } = serde_json::from_reader(std::io::BufReader::new(file))?;
    let results: Vec<_> = classes.into_iter().rev().map(|xs| structure_to_syntax(xs, &mut defined)).collect();

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
    let bpy_ops: TokenStream = ops_codegen(operators);

    let module = quote! {
        pub mod bpy {
            use serde::{ Deserialize, Serialize };
            use smartstring::alias::String;
            use crate::{ BpyPtr, PyArgs, invoke_bpy_setattr, invoke_bpy_getattr, invoke_bpy_callmethod, invoke_bpy_operator };
            use std::collections::HashMap;

            mod private {
                pub trait Sealed {}
            }

            impl private::Sealed for BpyPtr {}

            pub mod types {
                use super::*;
                fn get(ptr: &BpyPtr, key: &str) -> Option<BpyPtr> {
                    let args = PyArgs::arg1(ptr, key);
                    let result = invoke_bpy_callmethod("get", args);
                    let result: Option<BpyPtr> = serde_json::from_value(result).expect("TKTK");
                    result
                }

                fn keys(ptr: &BpyPtr) -> Vec<String> {
                    let args = PyArgs::new(ptr);
                    let result = invoke_bpy_callmethod("keys", args);
                    let result: Vec<String> = serde_json::from_value(result).expect("TKTK");
                    result
                }

                fn values(ptr: &BpyPtr) -> Vec<BpyPtr> {
                    let args = PyArgs::new(ptr);
                    let result = invoke_bpy_callmethod("values", args);
                    let result: Vec<BpyPtr> = serde_json::from_value(result).expect("TKTK");
                    result
                }

                fn items(ptr: &BpyPtr) -> Vec<(String, BpyPtr)> {
                    let args = PyArgs::new(ptr);
                    let result = invoke_bpy_callmethod("items", args);
                    let result: Vec<(String, BpyPtr)> = serde_json::from_value(result).expect("TKTK");
                    result
                }

                #results
            }

            #[derive(Deserialize)]
            struct BpyData {
                context: i64,
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

            pub mod data {
                use super::*;
                use extism_pdk;

                #bpy_data_impls
            }

            pub mod ops {
                use super::*;

                #bpy_ops
            }

            pub fn context() -> Box<dyn types::Context + Send + Sync> {
                Box::new(BpyPtr { ptr: load_bpy_data().context })
            }
        }
    };

    let syntree = syn::parse2(module)?;
    println!("{}", prettyplease::unparse(&syntree));

    Ok(())
}
