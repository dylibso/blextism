use extism_pdk::*;
use blender_extism_wasm_pdk::bpy::{self, types::NodeSocket};

fn scene_setup() {
}

fn link_nodes(
    node_tree: &dyn bpy::types::NodeTree,
    from: &dyn bpy::types::Node,
    to: &dyn bpy::types::Node,
    on_name: &str
) -> Option<()> {
    let from_mesh = from.outputs()?.get(on_name)?;
    let to_mesh = to.inputs()?.get(on_name)?;
    node_tree.links()?.new(from_mesh.to_bpy_ptr(), to_mesh.to_bpy_ptr(), None);
    Some(())
}

fn create_random_bool_value_node(
    node_tree: &dyn bpy::types::NodeTree,
    node_x_location: i32,
) -> Option<Box<dyn NodeSocket + Send + Sync>> {
    let (random_node, node_x_location) = create_node(
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
        None
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
    as_scale.keyframe_insert("default_value", None, Some(0.0), None, None);
    as_scale.set_default_value(Some(0.8));
    as_scale.keyframe_insert("default_value", None, Some(45.0), None, None);
    as_scale.set_default_value(Some(0.0));
    as_scale.keyframe_insert("default_value", None, Some(90.0), None, None);


    let links = node_tree.links()?;

    links.new(
        random_value_node_output_socket.to_bpy_ptr(),
        scale_elements_node.inputs()?.get("Selection")?.to_bpy_ptr(),
        None
    );

    links.new(
        socket.to_bpy_ptr(),
        scale_elements_node.inputs()?.get("Geometry")?.to_bpy_ptr(),
        None
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
        None
    );

    links.new(
        bottom_scale_elements_node.outputs()?.get("Geometry")?.to_bpy_ptr(),
        join_geometry_node.inputs()?.get("Geometry")?.to_bpy_ptr(),
        None
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
        None
    );

    links.new(
        join_geometry_node.outputs()?.get("Geometry")?.to_bpy_ptr(),
        out_node.inputs()?.get("Geometry")?.to_bpy_ptr(),
        None
    );
    Some(())
}

fn create_centerpiece() -> Option<()> {
    bpy::ops::mesh::primitive_plane_add(());

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
    let Some(cube) = bpy::data::objects().get("Cube") else {
        return Ok(())
    };

    if let Some(mut scale) = cube.scale() {
        extism_pdk::info!("scale={:?}", scale);
        scale[0] = 6.0;
        cube.set_scale(Some(scale.as_slice()));
    }

    eprintln!("hello world! {:?}", bpy::data::objects().items());
    eprintln!("but wait! {:?}", bpy::context().active_object());

    example_main();
    Ok(())
}
