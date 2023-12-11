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
    let objs = bpy::data::objects();

    let Some(cube) = bpy::data::objects().get("Cube") else {
        return Ok(())
    };

    if let Some(mut scale) = cube.get_scale() {
        extism_pdk::info!("scale={:?}", scale);
        scale[0] = 6.0;
        cube.set_scale(Some(scale.as_slice()));
    }

    eprintln!("hello world! {:?}", bpy::data::objects().items());
    eprintln!("but wait! {:?}", bpy::context().get_active_object());

    Ok(())
}
