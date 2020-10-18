use vek::{Extent2, Rgb};
use prima::geom::{BoundingRect, Line};
use prima::render::*;
use super::{Link, LinkType, IMG_SCALE};

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

    pub fn link_type(&self, index: usize) -> Option<LinkType> {
        if self.is_linked(index) {
            let link = self.links[index];
            Some(link.link_type)
        } else {
            None    
        }
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

impl Draw<f32> for Room {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        let boundingbox = BoundingRect {
                min: self.rect.min * IMG_SCALE as f32,
                max: self.rect.max * IMG_SCALE as f32,
            }.made_valid();

        boundingbox.into_rect().draw(image, colour);

        // for i in 0..self.intersects.len() {
        //     Line {
        //         start: self.intersects[i].line.start * IMG_SCALE as f32,
        //         end: self.intersects[i].line.end * IMG_SCALE as f32,
        //     }.draw(image, Rgb::new(0,0,255));
        // }
    }
}