use corale::core::GeoNum;
use corale::geom::Torus as CoraleTorus;
use vek::Vec3;
use super::{Arc, MPos, TPos};

pub struct Torus<T> where T: GeoNum {
    major: T,
    minor: T,
    pos: Vec3<T>
}

impl<T> CoraleTorus<T> for Torus<T> where T: GeoNum {
    fn major(&self) -> T {
        self.major
    }

    fn minor(&self) -> T {
        self.minor
    }

    fn center(&self) -> Vec3<T> {
        self.pos
    }
}

impl<T> Torus<T> where T: GeoNum {
    pub fn new(major: T, minor: T, pos: Vec3<T>) -> Self {
        Self {
            major,
            minor,
            pos,
        }
    }

    /// takes a TPos and converts it to world-space
    pub fn world_pos(&self, pos: TPos<T>) -> Vec3<T> {
        let t = pos.theta.to_f64().unwrap();
        let sin_t = T::from_f64(t.sin()).unwrap();
        let cos_t = T::from_f64(t.cos()).unwrap();
        let r = self.major() + self.minor() - pos.y;
        let x = r * sin_t;
        let y = r* cos_t;
        Vec3::new(x, y, pos.x) + self.pos
    }

    /// gets the flat dimensions of the given arc
    pub fn arc_bounds(&self, arc: &Arc<T>) {

    }

    pub fn arc_to_world(&self, arc: &Arc<T>, pos: MPos<T>, clamp: bool) {
        // get the arc bounds

        // inverse lerp the MPos into bounds, and clamp if nescisary
    }
}