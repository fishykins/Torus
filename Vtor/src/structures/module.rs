use corale::core::OrdNum;
use crate::geom::Torus;

#[derive(Clone)]
pub struct Module<'a, T> where T: OrdNum {
    torus: &'a Torus<T>,
}

impl<'a, T> Module<'a, T> where T: OrdNum {
    pub fn new(torus: &'a Torus<T>) -> Self {
        Self {
            torus,
        }
    }
}