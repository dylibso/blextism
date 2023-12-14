use extism_pdk::*;
use blextism::bpy::{self, types::NodeSocket};

// A port of https://github.com/CGArtPython/blender_plus_python/blob/main/geo_nodes/subdivided_triangulated_cube/subdivided_triangulated_cube_part_2_done.py

fn scene_setup() -> Option<()> {
    if bpy::context().active_object().and_then(|obj| obj.mode())? == "EDIT" {
        bpy::ops::object::editmode_toggle(());
    }

    for obj in bpy::data::objects().values() {
        obj.hide_set(false, ().into());
        obj.set_hide_select(Some(false));
        obj.set_hide_viewport(Some(false));
    }

    bpy::ops::object::select_all((
        ("action", "SELECT"),
    ));
    bpy::ops::object::delete(());

    let worlds = bpy::data::worlds();
    for world in worlds.values() {
        worlds.remove(world.to_bpy_ptr(), ().into());
    }
    bpy::ops::world::new(());

    let scene = bpy::context().scene()?;
    scene.set_world(Some(worlds.get("World")?.to_bpy_ptr()));
    let render = scene.render();
    render.set_engine(Some("CYCLES"));
    render.image_settings().set_file_format(Some("FFMPEG"));
    render.ffmpeg()?.set_codec(Some("H264"));
    render.set_filepath(Some("output"));
    render.set_fps(Some(30));
    render.set_resolution_x(Some(1280));
    render.set_resolution_y(Some(720));

    bpy::ops::outliner::orphans_purge((
        ("do_local_ids", true),
        ("do_linked_ids", true),
        ("do_recursive", true),
    ));

    scene.set_frame_end(Some(30 * 12));

    let world = worlds.get("World")?;
    if let Some(bg) = world.node_tree().and_then(|xs| xs.nodes()).and_then(|xs| xs.get("Background")) {
        let socket = bg.inputs()?.get("Color")?;
        let socket = Box::new(
            socket.to_bpy_ptr()
        ) as Box<dyn bpy::types::NodeSocketColor>;
        socket.set_default_value(Some(&[0., 0., 0., 1.]));
    }

    scene.set_frame_current(Some(1));
    scene.set_frame_start(Some(1));

    let light_data = bpy::data::lights().new("light", "POINT")?;
    let as_point_light_data = Box::new(
        light_data.to_bpy_ptr()
    ) as Box<dyn bpy::types::PointLight>;
    as_point_light_data.set_energy(Some(100.0));

    let light_object = bpy::data::objects().new("light", as_point_light_data.to_bpy_ptr())?;

    bpy::context().collection()?.objects()?.link(light_object.to_bpy_ptr());
    light_object.set_location(Some(&[2.0, 2.0, 1.333]));
    bpy::context().view_layer()?.objects()?.set_active(Some(light_object.to_bpy_ptr()));

    let camera_data = bpy::data::cameras().new("Camera")?;
    let camera_object = bpy::data::objects().new("Camera", camera_data.to_bpy_ptr())?;


    camera_object.set_location(Some(&[4.93136, -2.46555, 4.62837]));
    camera_object.set_rotation_euler(Some(&[0.9223401872, 0., 1.10716881]));

    scene.set_camera(Some(camera_object.to_bpy_ptr()));

    bpy::context().collection()?.objects()?.link(camera_object.to_bpy_ptr());
    let dg = bpy::context().evaluated_depsgraph_get()?;
    dg.update();



    Some(())
}

fn link_nodes(
    node_tree: &dyn bpy::types::NodeTree,
    from: &dyn bpy::types::Node,
    to: &dyn bpy::types::Node,
    on_name: &str
) -> Option<()> {
    let from_mesh = from.outputs()?.get(on_name)?;
    let to_mesh = to.inputs()?.get(on_name)?;
    node_tree.links()?.new(from_mesh.to_bpy_ptr(), to_mesh.to_bpy_ptr(), ().into());
    Some(())
}

fn create_random_bool_value_node(
    node_tree: &dyn bpy::types::NodeTree,
    node_x_location: i32,
) -> Option<Box<dyn NodeSocket + Send + Sync>> {
    let (random_node, _) = create_node(
        node_tree,
        "FunctionNodeRandomValue",
        node_x_location,
        0,
        -200
    )?;
    let as_func_node = Box::new(
        random_node.to_bpy_ptr()
    ) as Box<dyn bpy::types::FunctionNodeRandomValue>;
    as_func_node.set_data_type(Some("BOOLEAN"));

    random_node.outputs()?.values().into_iter().find(|xs| {
        let Some(ty) = xs.r#type() else { return false };
        ty.as_str() == "BOOLEAN"
    })
}

fn create_separate_geo_node(
    node_tree: &dyn bpy::types::NodeTree,
    node_x_location: i32,
    node_location_step: i32
) -> Option<(Box<dyn bpy::types::Node + Send + Sync>, i32)> {
    let random_value_node_output_socket = create_random_bool_value_node(node_tree, node_x_location)?;
    let (separate_geometry_node, node_x_location) = create_node(
        node_tree,
        "GeometryNodeSeparateGeometry",
        node_x_location,
        node_location_step,
        0
    )?;

    let as_geom_sep_node = Box::new(
        separate_geometry_node.to_bpy_ptr()
    ) as Box<dyn bpy::types::GeometryNodeSeparateGeometry>;
    as_geom_sep_node.set_domain(Some("FACE"));

    node_tree.links()?.new(
        random_value_node_output_socket.to_bpy_ptr(),
        separate_geometry_node.inputs()?.get("Selection")?.to_bpy_ptr(),
        ().into()
    )?;

    Some((separate_geometry_node, node_x_location))
}

fn create_scale_element_geo_node(
    node_tree: &dyn bpy::types::NodeTree,
    socket: &dyn bpy::types::NodeSocket,
    node_x_location: i32,
    node_y_location: i32,
) -> Option<(Box<dyn bpy::types::Node + Send + Sync>, i32)> {
    let random_value_node_output_socket = create_random_bool_value_node(node_tree, node_x_location)?;


    let (scale_elements_node, node_x_location) = create_node(
        node_tree,
        "GeometryNodeScaleElements",
        node_x_location,
        200,
        node_y_location
    )?;

    let scale = scale_elements_node.inputs()?.get("Scale")?;

    let as_scale = Box::new(
        scale.to_bpy_ptr()
    ) as Box<dyn bpy::types::NodeSocketFloat>;
    as_scale.set_default_value(Some(0.0));
    as_scale.keyframe_insert("default_value", (
        ("frame", 0.0),
    ).into());
    as_scale.set_default_value(Some(0.8));
    as_scale.keyframe_insert("default_value", (
        ("frame", 45.0),
    ).into());
    as_scale.set_default_value(Some(0.0));
    as_scale.keyframe_insert("default_value", (
        ("frame", 90.0),
    ).into());

    let links = node_tree.links()?;

    links.new(
        random_value_node_output_socket.to_bpy_ptr(),
        scale_elements_node.inputs()?.get("Selection")?.to_bpy_ptr(),
        ().into()
    );

    links.new(
        socket.to_bpy_ptr(),
        scale_elements_node.inputs()?.get("Geometry")?.to_bpy_ptr(),
        ().into()
    );

    Some((scale_elements_node, node_x_location))
}

fn separate_faces_and_animate_scale(
    node_tree: &dyn bpy::types::NodeTree,
    node_x_location: i32,
    node_location_step: i32,
) -> Option<(
    Box<dyn bpy::types::Node + Send + Sync>,
    Box<dyn bpy::types::Node + Send + Sync>,
    i32
)> {

    let (separate_geometry_node, node_x_location) = create_separate_geo_node(
        node_tree,
        node_x_location,
        node_location_step
    )?;

    let (top_scale_elements_node, node_x_location) = create_scale_element_geo_node(
        node_tree,
        &*separate_geometry_node.outputs()?.get("Selection")?,
        node_x_location,
        200
    )?;

    let (bottom_scale_elements_node, node_x_location) = create_scale_element_geo_node(
        node_tree,
        &*separate_geometry_node.outputs()?.get("Inverted")?,
        node_x_location,
        200
    )?;

    let fcurves = node_tree.animation_data()?.action()?.fcurves()?.values();
    for fcurve in fcurves {
        fcurve.modifiers()?.new("CYCLES");
    }

    let (join_geometry_node, node_x_location) = create_node(
        node_tree,
        "GeometryNodeJoinGeometry",
        node_x_location,
        node_location_step,
        0
    )?;

    let links = node_tree.links()?;

    links.new(
        top_scale_elements_node.outputs()?.get("Geometry")?.to_bpy_ptr(),
        join_geometry_node.inputs()?.get("Geometry")?.to_bpy_ptr(),
        ().into()
    );

    links.new(
        bottom_scale_elements_node.outputs()?.get("Geometry")?.to_bpy_ptr(),
        join_geometry_node.inputs()?.get("Geometry")?.to_bpy_ptr(),
        ().into()
    );

    Some((separate_geometry_node, join_geometry_node, node_x_location))
}

fn create_node(
    node_tree: &dyn bpy::types::NodeTree,
    type_name: &str,
    node_x_location: i32,
    node_location_step: i32,
    node_y_location: i32
) -> Option<(Box<dyn bpy::types::Node + Send + Sync>, i32)> {
    let node_obj = node_tree.nodes()?.new(type_name)?;
    node_obj.set_location(Some(&[node_x_location as f64, node_y_location as f64]));
    Some((node_obj, node_x_location + node_location_step))
}

fn update_geo_node_tree(node_tree: &dyn bpy::types::NodeTree) -> Option<()> {
    let out_node = node_tree.nodes()?.get("Group Output")?;
    let node_x_location = 0;
    let node_location_step_x = 300;

    let (mesh_cube_node, node_x_location) = create_node(
        node_tree,
        "GeometryNodeMeshCube",
        node_x_location,
        node_location_step_x,
        0
    )?;

    let (subdivide_mesh_node, node_x_location) = create_node(
        node_tree,
        "GeometryNodeSubdivideMesh",
        node_x_location,
        node_location_step_x,
        0
    )?;

    let lvl = Box::new(
        subdivide_mesh_node.inputs()?.get("Level")?.to_bpy_ptr()
    ) as Box<dyn bpy::types::NodeSocketInterfaceIntUnsigned>;

    lvl.set_default_value(Some(3));

    let (triangulate_node, node_x_location) = create_node(
        node_tree,
        "GeometryNodeTriangulate",
        node_x_location,
        node_location_step_x,
        0
    )?;

    let (split_edges_node, node_x_location) = create_node(
        node_tree,
        "GeometryNodeSplitEdges",
        node_x_location,
        node_location_step_x,
        0
    )?;

    let (separate_geometry_node, join_geometry_node, node_x_location) = separate_faces_and_animate_scale(
        node_tree,
        node_x_location,
        node_location_step_x
    )?;

    let mut pos = out_node.location()?;
    pos[0] = node_x_location as f64;
    out_node.set_location(Some(pos.as_slice()));

    link_nodes(node_tree, &*mesh_cube_node, &*subdivide_mesh_node, "Mesh"); 
    link_nodes(node_tree, &*subdivide_mesh_node, &*triangulate_node, "Mesh"); 
    link_nodes(node_tree, &*triangulate_node, &*split_edges_node, "Mesh");

    let links = node_tree.links()?;
    links.new(
        split_edges_node.outputs()?.get("Mesh")?.to_bpy_ptr(),
        separate_geometry_node.inputs()?.get("Geometry")?.to_bpy_ptr(),
        ().into()
    );

    links.new(
        join_geometry_node.outputs()?.get("Geometry")?.to_bpy_ptr(),
        out_node.inputs()?.get("Geometry")?.to_bpy_ptr(),
        ().into()
    );
    Some(())
}

fn create_centerpiece() -> Option<()> {
    bpy::ops::mesh::primitive_plane_add(());
    bpy::context().active_object()?.set_scale(Some(&[10., 10., 1.]));

    bpy::ops::mesh::primitive_plane_add(());
    bpy::context().active_object()?.set_location(Some(&[0., 0., 1.5]));

    bpy::ops::node::new_geometry_nodes_modifier(());
    let node_tree = bpy::data::node_groups().get("Geometry Nodes")?;
    update_geo_node_tree(&*node_tree);

    // TODO: we need to support kwargs for ops, sadly.
    bpy::ops::object::modifier_add((
        ("type", "SOLIDIFY"),
    ));

    bpy::context().active_object()?.modifiers()?.get("GeometryNodes")?.set_is_active(Some(true));
    Some(())
}

fn example_main() {
    scene_setup();
    create_centerpiece();
    bpy::ops::wm::save_as_mainfile((
        ("filepath", "foo.blend"),
    ));
}

#[plugin_fn]
pub fn example() -> FnResult<()> {
    example_main();
    Ok(())
}
