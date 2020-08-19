use corale::core::GeoNum;
use crate::geom::Torus;

#[derive(Clone)]
pub struct Module<'a, T> where T: GeoNum {
    torus: &'a Torus<T>,
}

impl<'a, T> Module<'a, T> where T: GeoNum {
    pub fn new(torus: &'a Torus<T>) -> Self {
        Self {
            torus,
        }
    }
}