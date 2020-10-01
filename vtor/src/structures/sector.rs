use prima::core::GeoNum;
use crate::geom::Arc;


pub struct Sector<T> where T: GeoNum {
    uid: usize,
    arc: Arc<T>,
}

impl<T> Sector<T> where T: GeoNum {
    pub fn new(uid: usize, arc_size: f64) -> Self {

        let size = T::from_f64(arc_size).unwrap();
        let i = T::from_usize(uid).unwrap();
        let theta = size * i;
        let arc = Arc::new(theta, size, T::one(), T::one());

        Self {
            uid,
            arc,
        }
    }

    pub fn arc(&self) -> &Arc<T> {
        &self.arc
    }

    pub fn uid(&self) -> usize {
        self.uid
    }
}