use vek::{Vec2};
use std::io::BufReader;
use crate::parse::parse_obj;
use crate::export::*;
use crate::station::ring_space::*;
use crate::mesh::*;

const LENGTH_K: f32 = 3.;
const WIDTH_K: f32 = 1.;
const HEIGHT_K: f32 = 1.;

#[derive(Clone)]
pub struct Module {
    index: usize,
    bounding_box: RBox,
}


impl Module {
    /// Creates a new module. 
    /// theta: the angle size of the segment's arc
    /// radius: the radius from the ring center to the middle of the module
    pub fn new(index: usize, arc: f32, radius: f32) -> Self {
        let angle = index as f32 * arc;
        let mut bounding_box = RBox::new(Vec2::zero(), angle, radius, arc, 0., 0.);
        let k = bounding_box.length() / LENGTH_K;
        bounding_box.set_size(k * WIDTH_K, k * HEIGHT_K);

        Self {
            index,
            bounding_box,
        }
    }

    pub fn build(&self, master_mesh: &Mesh) {
        let mut mesh = master_mesh.clone();
        self.bounding_box.map_mesh(&mut mesh);
        mesh.invert_z();
        let name = format!("module_{}", self.index);
        export_obj(mesh, &name, &name).unwrap();
    }
}