use corale::core::OrdNum;
use corale::geom::{BoundingBox, BoxCollider, Cube};
use super::Arc;


pub struct TorusSegment<T> where T: OrdNum {
    /// the arc of this segment
    arc: Arc<T>,
    /// radius of the outer torus
    radius: T,
}