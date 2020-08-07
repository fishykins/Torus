use vek::Vec3;
use crate::mesh::*;

pub fn default_cube(pos: Vec3<f64>, size: Vec3<f64>, mesh: &mut Mesh) {
    let size = size / 2.;
    let v0 = mesh.add_vertex(Vec3::new(pos.x + size.x, pos.y + size.y, pos.z - size.z));
    let v1 = mesh.add_vertex(Vec3::new(pos.x + size.x, pos.y - size.y, pos.z - size.z));
    let v2 = mesh.add_vertex(Vec3::new(pos.x + size.x, pos.y + size.y, pos.z + size.z));
    let v3 = mesh.add_vertex(Vec3::new(pos.x + size.x, pos.y - size.y, pos.z + size.z));
    let v4 = mesh.add_vertex(Vec3::new(pos.x - size.x, pos.y + size.y, pos.z - size.z));
    let v5 = mesh.add_vertex(Vec3::new(pos.x - size.x, pos.y - size.y, pos.z - size.z));
    let v6 = mesh.add_vertex(Vec3::new(pos.x - size.x, pos.y + size.y, pos.z + size.z));
    let v7 = mesh.add_vertex(Vec3::new(pos.x - size.x, pos.y - size.y, pos.z + size.z));
    mesh.add_face(vec![v2,v6,v4,v0]);
    mesh.add_face(vec![v7,v6,v2,v3]);
    mesh.add_face(vec![v5,v4,v6,v7]);
    mesh.add_face(vec![v7,v3,v1,v5]);
    mesh.add_face(vec![v3,v2,v0,v1]);
    mesh.add_face(vec![v1,v0,v4,v5]);
}