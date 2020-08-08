use vek::Vec3;

pub trait BoxCollider<T> {
    fn min(&self) -> Vec3<T>;
    fn max(&self) -> Vec3<T>;
    fn intersects(&self, other: &dyn BoxCollider<T>) -> bool;
    fn contains(&self, other: &dyn BoxCollider<T>) -> bool;
}