use vek::{Vec2};
use crate::station::ring_space::*;
use crate::mesh::*;
use crate::primatives::default_cube;
use noise::{NoiseFn, Perlin};

const LENGTH_K: Float = 3.;
const WIDTH_K: Float = 1.;
const HEIGHT_K: Float = 1.;

const AREA_INNER: (f64,f64,f64) = (0.5, 0.5, 1.);
const GRID_DIV: (usize,usize,usize) = (5,5,14);
const ROOM_THRESHOLD: f64 = 0.3;
const NOISE_FACTOR: f64 = 5.;

type Float = f64;

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
    pub fn new(index: usize, arc: Float, radius: Float, mesh: &Mesh) -> Self {
        let angle = index as Float * arc;
        let mut bounding_box = RBox::new(Vec2::zero(), angle, radius, arc, 0., 0.);
        let k = bounding_box.length() / LENGTH_K;
        bounding_box.set_size(k * WIDTH_K, k * HEIGHT_K);
        let mut new_mesh = mesh.clone();
        let name = format!("module_{}", index);
        new_mesh.set_name(name);

        let mut me = Self {
            index,
            bounding_box,
            mesh: new_mesh,
        };

        me.generate_rooms();
        me
    }

    pub fn generate_rooms(&mut self) {
        let perlin = Perlin::new();
        let step = Vec3::new(
            AREA_INNER.0 / GRID_DIV.0 as f64, 
            AREA_INNER.1 / GRID_DIV.1 as f64, 
            AREA_INNER.2 / GRID_DIV.2 as f64
        );

        let offset = Vec3::new(
            (1. - AREA_INNER.0) /2. + step.x / 2., 
            (1. - AREA_INNER.1) /2. + step.y / 2., 
            (1. - AREA_INNER.2) /2. + step.z / 2.
        );

        for x in 0..GRID_DIV.0 {
            for y in 0..GRID_DIV.1 {
                for z in 0..GRID_DIV.2 {
                    let pos = Vec3::new(
                        offset.x + step.x * x as f64, 
                        offset.y + step.y * y as f64, 
                        offset.z + step.z * z as f64
                    );
                    let val = perlin.get([pos.x * NOISE_FACTOR, pos.y * NOISE_FACTOR, pos.z * NOISE_FACTOR]);
                    //println!("{}: [{},{},{}] = {}", self.mesh.name().unwrap(), pos.x, pos.y, pos.z, val);
                    if val > ROOM_THRESHOLD {
                        default_cube(pos, step, &mut self.mesh);
                    }
                }
            }
        }
        println!("step: {}", step);
        println!("offset: {}", offset);
        
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