use vek::{Extent2};
use prima::geom::{BoundingRect};
use super::{Link, LinkType};

#[derive(Clone)]
pub struct Room {
    pub rect: BoundingRect<f32>,
    pub value: f32,
    links: Vec<Link>,
}

impl Room {
    pub fn new(rect: BoundingRect<f32>) -> Self {
        Self {
            rect,
            links: Vec::new(),
            value: 0.
        }
    }

    pub fn size(&self) -> Extent2<f32> {
        self.rect.size()
    }

    pub fn connected(&self) -> Vec<usize> {
        self.links.iter().map(|x| x.target).collect()
    }

    pub fn links(&self) -> Vec<&Link> {
        let mut array = Vec::new();
        for i in &self.links {
            array.push(i);
        }
        array
    }

    pub fn is_linked(&self, other: usize) -> bool {
        self.connected().contains(&other)
    }

    pub fn link(&mut self, other: usize, link_type: LinkType) {
        let link = Link::new(other, link_type);
        if !self.links.contains(&link) {
            self.links.push(link);
        }
    }
}