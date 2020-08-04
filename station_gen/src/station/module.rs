use vek::{Vec2, Vec3, Vec4};
use crate::mesh::*;
use crate::station::cplane::CPlane;

const LENGTH_K: f32 = 3.;
const WIDTH_K: f32 = 2.;
const HEIGHT_K: f32 = 1.;

pub struct Interior {
    k: f32,
    arc: f32,
    radius: f32,
    floor: CPlane,
    ceiling: CPlane,
}

pub struct Exterior {

}

pub struct Module {
    center: Vec2<f32>,
    interior: Interior,
    exterior: Exterior,
}

impl Interior {
    /// arc: the circle circumference this will occupy.
    /// radius: the distance from the center of the station to the center of this module
    pub fn new(arc: f32, radius: f32) -> Self {
        let theta = (- arc / 2., arc / 2.);
        let front = Vec2::new(radius * theta.1.cos(), radius * theta.1.sin());
        let back = Vec2::new(radius * theta.0.cos(), radius * theta.0.sin());
        let k = back.distance(front) / LENGTH_K;
        let radius_floor = radius + k * HEIGHT_K / 2.;
        let radius_ceiling = radius_floor - (k * HEIGHT_K);

        Self {
            k,
            arc,
            radius,
            floor: CPlane::new(Vertex::zero(), arc, radius_floor, k * WIDTH_K),
            ceiling: CPlane::new(Vertex::zero(), arc, radius_ceiling, k * WIDTH_K),
        }
    }

    /// Gets a plane that covers the entire module floor space at a given height from the base
    pub fn get_cplane(&self, height: f32) -> CPlane {
        CPlane::new(Vertex::zero(), self.arc, self.radius - height + (self.k * HEIGHT_K / 2.), self.k * WIDTH_K)
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
            center,
            interior,
            exterior,
        }
    }
}