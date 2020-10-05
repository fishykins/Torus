#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LinkType {
    Direct,
    Tunnel,
    Bridge,
    Main,
}


#[derive(Clone, Debug, Eq, PartialEq)]
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