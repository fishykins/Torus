use std::fs::File;
use std::io::prelude::*;
use crate::mesh::*;

pub fn export_obj(mesh: Mesh, obj_name: &str, file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("../bin/renders/{}.obj", file_name))?;
    let mut data = Vec::new();
    data.push(format!("# Generated for use in Torus"));
    data.push(format!("mtllib {}.mtl", file_name));
    data.push(format!("o {}", obj_name));

    for vert in mesh.verticies().iter() {
        data.push(format!("v {} {} {}", vert.x, vert.z, vert.y));
    }

    for face in mesh.faces().iter() {
        let mut list = Vec::new();
        for v in face.iter() {
            list.push(format!("{}", v));
        };
        data.push(format!("f {}", list.join(" ")));
    }

    file.write(data.join("\n").as_bytes())?;
    Ok(())
}