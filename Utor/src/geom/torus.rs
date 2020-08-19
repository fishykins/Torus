use corale::core::OrdNum;
use corale::geom::Torus as Tor;
use vek::Vec3;


pub struct Torus<T> where T: OrdNum {
    major: T,
    minor: T,
    center: Vec3<T>,
    
}