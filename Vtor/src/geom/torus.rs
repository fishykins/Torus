use corale::core::GeoNum;
use corale::geom::Torus as CoraleTorus;
use vek::Vec3;
use super::{Arc, TPos, TBounds};

pub struct Torus<T> where T: GeoNum {
    major: T,
    minor: T,
    pos: Vec3<T>,
    circumference: T,
    circumference_outer: T,
    circumference_inner: T,
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
        let pi2 = T::from_f64(std::f64::consts::PI * 2.).unwrap();
        let circumference = pi2 * (major);
        let circumference_outer = pi2 * (major + minor);
        let circumference_inner = pi2 * (major - minor);
        Self {
            major,
            minor,
            pos,
            circumference,
            circumference_outer,
            circumference_inner,
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

    pub fn mbounds<'a>(&self, arc: &'a Arc<T>) {

    }
}