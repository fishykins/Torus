use vek::{Vec2};
use crate::station::ring_space::*;
use crate::mesh::*;

const LENGTH_K: f32 = 3.;
const WIDTH_K: f32 = 1.;
const HEIGHT_K: f32 = 1.;

#[derive(Clone)]
pub struct Module {
    index: usize,
    bounding_box: RBox,
    mesh: Mesh,
}


impl Module {
    /// Creates a new module. 
    /// theta: the angle size of the segment's arc
    /// radius: the radius from the ring center to the middle of the module
    /// mesh: parent mesh to clone from. 
    pub fn new(index: usize, arc: f32, radius: f32, mesh: &Mesh) -> Self {
        let angle = index as f32 * arc;
        let mut bounding_box = RBox::new(Vec2::zero(), angle, radius, arc, 0., 0.);
        let k = bounding_box.length() / LENGTH_K;
        bounding_box.set_size(k * WIDTH_K, k * HEIGHT_K);
        let new_mesh = mesh.clone();
        mesh.set_name(format!("module_{}", index));

        Self {
            index,
            bounding_box,
            mesh: new_mesh,
        }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    pub fn build(&self) -> Mesh {
        let mut mesh = self.mesh.clone();
        self.bounding_box.map_mesh(&mut mesh);
        mesh.invert_z();
        mesh
        //export_obj(mesh, &name, &name).unwrap();
    }

    pub fn index(&self) -> usize {
        self.index
    }

}