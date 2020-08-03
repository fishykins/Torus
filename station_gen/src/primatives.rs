use vek::Vec3;
use crate::mesh::*;

pub fn default_cube(size: f32) -> Mesh {
    let mut mesh = Mesh::new();
    mesh.add_vertex(Vec3::new(size,size, -size));
    mesh.add_vertex(Vec3::new(size,-size, -size));
    mesh.add_vertex(Vec3::new(size,size, size));
    mesh.add_vertex(Vec3::new(size,-size, size));
    mesh.add_vertex(Vec3::new(-size,size, -size));
    mesh.add_vertex(Vec3::new(-size,-size, -size));
    mesh.add_vertex(Vec3::new(-size,size, size));
    mesh.add_vertex(Vec3::new(-size,-size, size));
    mesh.add_face(vec![1,5,7,3]);
    mesh.add_face(vec![4,3,7,8]);
    mesh.add_face(vec![8,7,5,6]);
    mesh.add_face(vec![6,2,4,8]);
    mesh.add_face(vec![2,1,3,4]);
    mesh.add_face(vec![6,5,1,2]);
    mesh
}