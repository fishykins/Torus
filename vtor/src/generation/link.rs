use super::intersect::IntersectRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LinkType {
    Direct(IntersectRef),
    Tunnel,
    Bridge,
    Main,
}


#[derive(Clone, Debug, Eq, PartialEq)]
/// Links are intended to be used as connection nodes between two rooms.
pub struct Link {
    pub link_type: LinkType,
    pub target: usize,
}

impl Link {
    pub fn new(target: usize, link_type: LinkType) -> Self {
        Self {
            link_type,
            target,
        }
    }
}
