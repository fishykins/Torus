use corale::core::GeoNum;
use vek::Vec3;

/// Represents a position from the Torus's "walking edge" (outer edge with "up" towards the center of the torus)
/// Theta is the angle of the point from the center of the torus.
/// x is horizontal offset, with 0 being the center.
/// y is height offset from "floor", where floor is the outer edge. 
#[derive(Clone, Copy, Debug)]
pub struct TPos<T> where T: GeoNum {
    pub x: T,
    pub y: T,
    pub theta: T,
}

impl<T> TPos<T> where T: GeoNum {
    pub fn new(x: T, y: T, theta: T) -> Self {
        Self {
            x,
            y,
            theta,
        }
    }

    pub fn theta(theta: T) -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            theta,
        }
    }
}

impl<T> From<Vec3<T>> for TPos<T> where T: GeoNum {
    fn from(pos: Vec3<T>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            theta: pos.z,
        }
    }
}