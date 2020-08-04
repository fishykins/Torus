use vek::{Vec2, Vec3, Vec4};
use crate::mesh::*;
use crate::station::ring_space::*;

const LENGTH_K: f32 = 3.;
const WIDTH_K: f32 = 1.;
const HEIGHT_K: f32 = 1.;

#[derive(Clone)]
pub struct Interior {
    bounding_box: RBox,
}

#[derive(Clone)]
pub struct Exterior {

}

#[derive(Clone)]
pub struct Module {
    arc: f32,
    center: Vec2<f32>,
    interior: Interior,
    exterior: Exterior,
}

impl Interior {
    /// arc: the circle circumference this will occupy.
    /// radius: the distance from the center of the station to the center of this module
    pub fn new(arc: f32, radius: f32) -> Self {
        let mut bounding_box = RBox::new(Vec2::zero(), radius, arc, 0., 0.);
        let k = bounding_box.length() / LENGTH_K;
        bounding_box.set_size(k * WIDTH_K, k * HEIGHT_K);
        Self {
            bounding_box,
        }
    }
}

impl Exterior {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Module {
    /// Creates a new module. 
    /// theta: the angle size of the segment's arc
    /// radius: the radius from the ring center to the middle of the module
    pub fn new(theta: f32, radius: f32) -> Self {
        let center = Vec2::new(radius, 0.);
        //Build structs
        let interior = Interior::new(theta, radius);
        let exterior = Exterior::new();
        Self {
            arc: theta,
            center,
            interior,
            exterior,
        }
    }

    pub fn arc(&self) -> f32 {
        self.arc.clone()
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::parse::parse_obj;
    use crate::export::*;
    use crate::station::ring_space::*;
    use vek::*;

    #[test]
    fn module_test() {
        let file = File::open(format!("assets/module.obj")).unwrap();
        let input = BufReader::new(file);
        let mut mesh = parse_obj(input).unwrap();
        let RBox = RBox::new(Vec2::zero(), 32., (2. * std::f64::consts::PI as f32) / 16., 8., 8.);
        RBox.map_mesh(&mut mesh);
        mesh.normalize(Vec3::new(0., 32., 0.));
        mesh.invert_z();
        export_obj(mesh, "module", "module_test").unwrap();
    }
}