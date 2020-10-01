use prima::core::{GeoNum, maths};
use prima::geom::{BoundingBox};
use vek::{Vec3};
use super::{Arc, TPos};


pub struct Torus<T> where T: GeoNum {
    major: T,
    minor: T,
    pos: Vec3<T>,
}

#[allow(dead_code)]
impl<T> Torus<T> where T: GeoNum {
    pub fn new(major: T, minor: T, pos: Vec3<T>) -> Self {
        Self {
            major,
            minor,
            pos,
        }
    }

    fn major(&self) -> T {
        self.major
    }

    fn minor(&self) -> T {
        self.minor
    }

    fn center(&self) -> Vec3<T> {
        self.pos
    }

    /// helper function to ensure all length calculations are the same
    pub fn arc_length(&self, arc: &Arc<T>) -> T {
        arc.circ() * self.major
    }

    /// makes a BoundingBox that represents the given arc in flat projection. Useful for working with real units of measurment prior to being normalized
    pub fn make_arc_bbox(&self, arc: &Arc<T>) -> BoundingBox<T> {
        let width = arc.size().x;
        let height = arc.size().y;
        let depth = self.arc_length(arc);
        let max = Vec3::new(width, height, depth);
        BoundingBox {
            min: Vec3::zero(),
            max,
        }
    }

    /// takes a TPos and converts it to world-space
    pub fn tpos_to_world(&self, pos: TPos<T>) -> Vec3<T> {
        let t = pos.theta.to_f64().unwrap();
        let sin_t = T::from_f64(t.sin()).unwrap();
        let cos_t = T::from_f64(t.cos()).unwrap();
        let r = self.major() + self.minor() - pos.y;
        let x = r * sin_t;
        let y = r* cos_t;
        Vec3::new(x, y, pos.x) + self.pos
    }

    pub fn vec3_to_world(&self, pos: Vec3<T>, arc: &Arc<T>) -> Vec3<T> {

        // convert z to theta
        let zi = maths::inverse_lerp(T::zero(), self.arc_length(arc), pos.z);
        let theta = arc.lerpc(zi);

        // move x origin from far left to center
        let width = arc.size().x  / (T::one() + T::one());
        let x = maths::clamp(-width, width, width - pos.x);

        //y is the same, just clamped
        let y = maths::clamp(T::zero(), arc.size().y, pos.y);

        // construct TPos and return
        self.tpos_to_world(TPos::new(x, y, theta))
    }
}