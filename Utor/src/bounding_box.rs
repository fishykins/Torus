use crate::box_collider::BoxCollider;
use vek::Vec3;

#[derive(PartialEq, Clone)]
pub struct BoundingBox<T> {
    bounds: Vec3<(T, T)>
}

impl<T> BoundingBox<T> {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self {
            bounds: min.zip(max),
        }
    }
}

impl BoxCollider<i8> for BoundingBox<i8> {
    fn min(&self) -> Vec3<i8> {
        self.bounds.map(|a| a.0)
    }

    fn max(&self) -> Vec3<i8> {
        self.bounds.map(|a| a.1)
    }

    fn contains(&self, other: &dyn BoxCollider<i8>) -> bool {
        let c1 = self.min().x < other.min().x;
        let c2 = self.max().x > other.max().x;
        let c3 = self.min().y < other.min().y;
        let c4 = self.max().y > other.max().y;
        let c5 = self.min().z < other.min().z;
        let c6 = self.max().z > other.max().z;

        c1 && c2 && c3 && c4 && c5 && c6
    }

    fn intersects(&self, other: &dyn BoxCollider<i8>) -> bool {
        let c1 = self.min().x > other.max().x;
        let c2 = self.max().x < other.min().x;
        let c3 = self.min().y > other.max().y;
        let c4 = self.max().y < other.min().y;
        let c5 = self.min().z > other.max().z;
        let c6 = self.max().z < other.min().z;

        !c1 && !c2 && !c3 && !c4 && !c5 && !c6
    }
}