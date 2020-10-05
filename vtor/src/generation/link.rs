
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Link {
    pub direct: bool,
    pub target: usize,
}

impl Link {
    pub fn new(target: usize, direct: bool) -> Self {
        Self {
            direct,
            target,
        }
    }
}