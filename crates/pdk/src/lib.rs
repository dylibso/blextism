mod bindings;

use extism_pdk::*;
use crate::bindings::{ bpy::PyArgs, bpy };


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


#[plugin_fn]
pub fn example() -> FnResult<()> {
    let input = extism_pdk::input_bytes();
    let bpy_ptr: bpy::BpyPtr = match serde_json::from_slice(input.as_slice()) {
        Ok(bpy) => bpy,
        Err(e) => {
            extism_pdk::error!("input={:?}", input.as_slice());
            extism_pdk::error!("{:?}", e);
            return Ok(())
        }
    };

    let as_object = Box::new(bpy_ptr) as Box<dyn bpy::types::Object>;

    if let Some(mut scale) = as_object.get_scale() {
        extism_pdk::info!("scale={:?}", scale);
        scale[0] = 6.0;
        as_object.set_scale(Some(scale.as_slice()));
    }

    eprintln!("hello world!");
    // broken:
    as_object.get_modifiers();

    let objs = bpy::data::objects();

    eprintln!("got {:?}", objs.get("Cube"));

    Ok(())
}
