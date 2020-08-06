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
    mesh.add_face(vec![0,4,6,2]);
    mesh.add_face(vec![3,2,6,7]);
    mesh.add_face(vec![7,6,4,5]);
    mesh.add_face(vec![5,1,3,7]);
    mesh.add_face(vec![1,0,2,3]);
    mesh.add_face(vec![5,4,0,1]);
    mesh
}