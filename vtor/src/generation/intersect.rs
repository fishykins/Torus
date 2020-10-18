use crate::geom::Compass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Intersect {
    pub edge: Compass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntersectRef {
    pub index: usize,
    pub edge: Compass,
}