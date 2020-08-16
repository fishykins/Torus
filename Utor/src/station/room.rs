use corale::geom::{BoxCollider, Cube};
use corale::core::{GridNum};
use crate::GrowableBox;
use vek::Vec3;

#[derive(Clone)]
pub struct Room<T> where T: GridNum {
    pub bounding_box: GrowableBox<T>,
}

impl<T> Room<T> where T: GridNum {
    pub fn new(bounding_box: GrowableBox<T>) -> Self {
        Self {
            bounding_box,
        }
    }
}

impl<T> Cube<T> for Room<T> where T: GridNum {
    fn min(&self) -> Vec3<T> {
        self.bounding_box.min()
    }

    fn max(&self) -> Vec3<T> {
        self.bounding_box.max()
    }
}

impl<T> BoxCollider<T> for Room<T> where T: GridNum {
    fn contains(&self, other: &dyn Cube<T>) -> bool {
        self.bounding_box.contains(other)
    }

    fn intersects(&self, other: &dyn Cube<T>) -> bool {
        self.bounding_box.intersects(other)
    }

    fn contains_point(&self, point: Vec3<T>) -> bool {
        let c1 = self.min().x <= point.x;
        let c2 = self.max().x >= point.x;
        let c3 = self.min().y <= point.y;
        let c4 = self.max().y >= point.y;
        let c5 = self.min().z <= point.z;
        let c6 = self.max().z >= point.z;

        c1 && c2 && c3 && c4 && c5 && c6
    }
}