use crate::box_collider::BoxCollider;
use crate::growable_box::*;
use vek::Vec3;

#[derive(Clone)]
pub struct Room {
    pub bounding_box: GrowableBox,
}

impl Room {
    pub fn new(bounding_box: GrowableBox) -> Self {
        Self {
            bounding_box,
        }
    }
}

impl BoxCollider<i8> for Room {
    fn min(&self) -> Vec3<i8> {
        self.bounding_box.min()
    }

    fn max(&self) -> Vec3<i8> {
        self.bounding_box.max()
    }

    fn contains(&self, other: &dyn BoxCollider<i8>) -> bool {
        self.bounding_box.contains(other)
    }

    fn intersects(&self, other: &dyn BoxCollider<i8>) -> bool {
        self.bounding_box.intersects(other)
    }
}