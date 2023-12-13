mod bindings;

use std::{collections::HashMap, str::FromStr};

use extism_pdk::*;
use serde::{ Deserialize, Serialize };
use smartstring::alias::String;
pub use crate::bindings::bpy;

#[derive(Serialize, Deserialize, Clone)]
pub struct BpyPtr {
    #[serde(rename = "@ptr")]
    ptr: i64,
}

impl std::fmt::Debug for BpyPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let args = PyArgs::new(self);
        let result: Option<String> = serde_json::from_value(invoke_bpy_callmethod("__repr__", args)).ok().flatten();
        f.debug_struct("BpyPtr")
            .field("__repr__", &result)
            .finish()
    }
}


#[derive(Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Kwargs(HashMap<String, serde_json::Value>);

#[derive(Serialize, Deserialize, Default)]
pub struct PyArgs {
    #[serde(rename = "self")]
    target: Option<BpyPtr>,
    args: Option<Vec<serde_json::Value>>,
    kwargs: Option<Kwargs>,
}

impl PyArgs {
    fn new(target: &BpyPtr) -> Self {
        Self {
            target: Some(target.clone()),
            ..Default::default()
        }
    }

    fn arg1(target: &BpyPtr, args: impl Serialize) -> Self {
        let value = serde_json::to_value(args).expect("pyarg must be serializable");
        Self {
            target: Some(target.clone()),
            args: Some(vec![value]),
            ..Default::default()
        }
    }

    fn argv(target: Option<&BpyPtr>, args: Vec<serde_json::Value>, kwargs: Option<Kwargs>) -> Self {
        Self {
            target: target.cloned(),
            args: Some(args),
            kwargs
        }
    }
}

macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!(T1);
        $name!(T1, T2);
        $name!(T1, T2, T3);
        $name!(T1, T2, T3, T4);
        $name!(T1, T2, T3, T4, T5);
        $name!(T1, T2, T3, T4, T5, T6);
        $name!(T1, T2, T3, T4, T5, T6, T7);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    };
}

macro_rules! impl_from_for_pyargs {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<$($ty,)*> From<($((&str, $ty),)*)> for Kwargs
        where
            $( $ty: Serialize, )*
        {
            fn from(value: ($((&str, $ty),)*)) -> Self {
                let ($($ty,)*) = value;
                let mut hm = HashMap::new();
                $(
                    hm.insert(String::from_str($ty.0).unwrap(), serde_json::to_value($ty.1).unwrap());
                )*
                Self(hm)
            }
        }
    }
}

all_the_tuples!(impl_from_for_pyargs);

impl From<()> for Kwargs {
    fn from(_value: ()) -> Self {
        Default::default()
    }
}

impl<S: Into<String>, T: Serialize> From<HashMap<S, T>> for Kwargs {
    fn from(value: HashMap<S, T>) -> Self {
        let hm: HashMap<_, _> = value.into_iter().map(|(k, v)| {
            (k.into(), serde_json::to_value(v).unwrap())
        }).collect();
        Kwargs(hm)
    }
}

#[host_fn("chrisdickinson:blender/bpy")]
extern "ExtismHost" {
    fn bpy_setattr(method: &str, args: Json<PyArgs>);
    fn bpy_getattr(method: &str, args: Json<PyArgs>) -> Json<serde_json::Value>;
    fn bpy_callmethod(method: &str, args: Json<PyArgs>) -> Json<serde_json::Value>;
    fn bpy_operator(opmod: &str, method: &str, args: Json<PyArgs>) -> Json<serde_json::Value>;
}

pub(crate) fn invoke_bpy_getattr(method: &str, args: PyArgs) -> serde_json::Value {
    unsafe {
        let result = bpy_getattr(method, Json(args)).unwrap().into_inner();
        extism_pdk::info!("{}", serde_json::to_string(&result).unwrap());
        result
    }
}

pub(crate) fn invoke_bpy_setattr(method: &str, args: PyArgs) {
    unsafe { bpy_setattr(method, Json(args)).unwrap() };
}

pub(crate) fn invoke_bpy_callmethod(method: &str, args: PyArgs) -> serde_json::Value {
    unsafe {
        let result = bpy_callmethod(method, Json(args)).unwrap().into_inner();
        extism_pdk::info!("{}", serde_json::to_string(&result).unwrap());
        result
    }
}

pub(crate) fn invoke_bpy_operator(opmod: &str, operator: &str, args: PyArgs) -> serde_json::Value {
    unsafe {
        let result = bpy_operator(opmod, operator, Json(args)).unwrap().into_inner();
        extism_pdk::info!("{}", serde_json::to_string(&result).unwrap());
        result
    }
}
