use prima::core::GeoNum;
use prima::geom::BoundingBox;
use crate::geom::{Arc};

#[derive(Clone)]
pub struct Module<T> where T: GeoNum {
    uid: usize,
    arc: Arc<T>,
    bbox: BoundingBox<T>,
}

impl<T> Module<T> where T: GeoNum {
    pub fn new(uid: usize, arc: Arc<T>, bbox: BoundingBox<T>) -> Self {
        Self {
            uid,
            arc,
            bbox,
        }
    }
}