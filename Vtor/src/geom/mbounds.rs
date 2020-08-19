use corale::core::GeoNum;
use corale::geom::{Cube, BoxCollider};
use vek::Vec3;

use super::Arc;

/// a bounding box that represents an arc in flat projection. 
/// Unlike TPos, the origin point is the bottom left point of the box for every axis
pub struct MBounds<'a, T> where T: GeoNum {
    min: Vec3<T>,
    max: Vec3<T>,
    arc: &'a Arc<T>,
}

impl<'a, T> MBounds<'a, T> where T: GeoNum {
    pub fn new(min: Vec3<T>, max: Vec3<T>, arc: &'a Arc<T>) -> Self {
        Self {
            min,
            max,
            arc,
        }
    }
}

impl<'a, T> BoxCollider<T> for MBounds<'a, T> where T: GeoNum {
    fn contains(&self, other: &dyn Cube<T>) -> bool {
        let c1 = self.min.x <= other.min().x;
        let c2 = self.max.x >= other.max().x;
        let c3 = self.min.y <= other.min().y;
        let c4 = self.max.y >= other.max().y;
        let c5 = self.min.z <= other.min().z;
        let c6 = self.max.z >= other.max().z;

        c1 && c2 && c3 && c4 && c5 && c6
    }

    fn intersects(&self, other: &dyn Cube<T>) -> bool {
        let c1 = self.min.x > other.max().x;
        let c2 = self.max.x < other.min().x;
        let c3 = self.min.y > other.max().y;
        let c4 = self.max.y < other.min().y;
        let c5 = self.min.z > other.max().z;
        let c6 = self.max.z < other.min().z;

        !c1 && !c2 && !c3 && !c4 && !c5 && !c6
    }

    fn contains_point(&self, point: Vec3<T>) -> bool {
        let c1 = self.min.x <= point.x;
        let c2 = self.max.x >= point.x;
        let c3 = self.min.y <= point.y;
        let c4 = self.max.y >= point.y;
        let c5 = self.min.z <= point.z;
        let c6 = self.max.z >= point.z;

        c1 && c2 && c3 && c4 && c5 && c6
    }
}