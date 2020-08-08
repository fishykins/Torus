use crate::mesh::Mesh;

pub trait MeshModifier {
    fn apply(&self, mesh: &Mesh) -> Mesh;
}