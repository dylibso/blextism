use extism_pdk::*;
use blender_extism_wasm_pdk::bpy;

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
