use vek::Vec3;
use corale::core::OrdNum;

pub struct RPos<T> where T: OrdNum {
    a: T,
    b: T,
    r: T,
}