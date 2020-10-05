use vek::{Extent2};
use prima::geom::{BoundingRect};
use super::Link;

#[derive(Clone)]
pub struct Room {
    pub rect: BoundingRect<f32>,
    pub main: bool,
    links: Vec<Link>,
}

impl Room {
    pub fn new(rect: BoundingRect<f32>) -> Self {
        Self {
            rect,
            links: Vec::new(),
            main: false,
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

    pub fn link(&mut self, other: usize, direct: bool) {
        let link = Link::new(other, direct);
        if !self.links.contains(&link) {
            self.links.push(link);
        }
    }
}