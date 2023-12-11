mod bindings;

use extism_pdk::*;
pub use crate::bindings::{ bpy::PyArgs, bpy };

#[host_fn("chrisdickinson:blender/bpy")]
extern "ExtismHost" {
    fn bpy_setattr(method: &str, args: Json<PyArgs>);
    fn bpy_getattr(method: &str, args: Json<PyArgs>) -> Json<serde_json::Value>;
    fn bpy_callmethod(method: &str, args: Json<PyArgs>) -> Json<serde_json::Value>;
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
