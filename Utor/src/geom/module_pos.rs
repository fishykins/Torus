use vek::Vec3;
use corale::core::OrdNum;
use super::Arc;

pub struct MPos<T> where T: OrdNum {
    pub x: T,
    pub y: T,
    pub z: T,
    w: T,
    h: T,
    arc: Arc<T>,
}

impl<T> MPos<T> where T: OrdNum {
    pub fn new() {
        
    }
}