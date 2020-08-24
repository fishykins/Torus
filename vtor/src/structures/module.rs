use corale::core::GeoNum;
use crate::geom::{Arc};

#[derive(Clone)]
pub struct Module<T> where T: GeoNum {
    uid: usize,
    arc: Arc<T>,
}

impl<T> Module<T> where T: GeoNum {
    pub fn new(uid: usize, arc: Arc<T>) -> Self {
        Self {
            uid,
            arc,
        }
    }
}