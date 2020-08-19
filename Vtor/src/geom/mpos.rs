use corale::core::GeoNum;
use vek::Vec3;

/// Represents a position within a rotated bounding-box, such as a module. Allows for quick and dirty visualisation
pub struct MPos<T> where T: GeoNum {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> MPos<T> where T: GeoNum {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

impl<T> From<Vec3<T>> for MPos<T> where T: GeoNum {
    fn from(pos: Vec3<T>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }
    }
}