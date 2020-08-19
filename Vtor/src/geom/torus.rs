use corale::core::OrdNum;
use corale::geom::Torus as CoraleTorus;
use vek::Vec3;

pub struct Torus<T> where T: OrdNum {
    major: T,
    minor: T,
    pos: Vec3<T>
}

impl<T> CoraleTorus<T> for Torus<T> where T: OrdNum {
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

impl<T> Torus<T> where T: OrdNum {
    pub fn new(major: T, minor: T, pos: Vec3<T>) -> Self {
        Self {
            major,
            minor,
            pos,
        }
    }
}