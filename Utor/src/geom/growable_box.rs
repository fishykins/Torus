use corale::geom::*;
use corale::core::{Direction, GridNum};
use vek::Vec3;

#[derive(PartialEq, Clone)]
pub struct GrowableBox<T> where T: GridNum {
    bounds: Vec3<(T, T)>,
}

impl<T> GrowableBox<T> where T: GridNum {
    pub fn new(pos: Vec3<T>) -> Self {
        Self {
            bounds: pos.zip(pos),
        }
    }

    pub fn pos(&self) -> Vec3<T> {
        let min: Vec3<T> = self.min().map(|z| z as T);
        min + self.size()
    }

    pub fn size(&self) -> Vec3<T> {
        let min: Vec3<T> = self.min().map(|z| z as T);
        let max: Vec3<T> = self.max().map(|z| z as T);
        max - min
        
    }

    pub fn expand(&mut self, direction: &Direction, amount: T) {
        match direction {
            Direction::Front => self.bounds.z.1 += amount,
            Direction::Back => self.bounds.z.0 -= amount,
            Direction::Right => self.bounds.x.1 += amount,
            Direction::Left => self.bounds.x.0 -= amount,
            Direction::Up => self.bounds.y.1 += amount,
            Direction::Down => self.bounds.y.0 -= amount,
            _ => self.twiddle_thumbs(),
        };
    }

    fn twiddle_thumbs(&self) {

    }
}

impl<T> Cube<T> for GrowableBox<T> where T: GridNum {
    fn min(&self) -> Vec3<T> {
        self.bounds.map(|a| a.0)
    }

    fn max(&self) -> Vec3<T> {
        self.bounds.map(|a| a.1)
    }
}

impl<T> BoxCollider<T> for GrowableBox<T> where T: GridNum {
    fn contains(&self, other: &dyn Cube<T>) -> bool {
        let c1 = self.min().x <= other.min().x;
        let c2 = self.max().x >= other.max().x;
        let c3 = self.min().y <= other.min().y;
        let c4 = self.max().y >= other.max().y;
        let c5 = self.min().z <= other.min().z;
        let c6 = self.max().z >= other.max().z;

        c1 && c2 && c3 && c4 && c5 && c6
    }

    fn intersects(&self, other: &dyn Cube<T>) -> bool {
        let c1 = self.min().x > other.max().x;
        let c2 = self.max().x < other.min().x;
        let c3 = self.min().y > other.max().y;
        let c4 = self.max().y < other.min().y;
        let c5 = self.min().z > other.max().z;
        let c6 = self.max().z < other.min().z;

        !c1 && !c2 && !c3 && !c4 && !c5 && !c6
    }

    fn contains_point(&self, point: Vec3<T>) -> bool {
        let c1 = self.min().x <= point.x;
        let c2 = self.max().x >= point.x;
        let c3 = self.min().y <= point.y;
        let c4 = self.max().y >= point.y;
        let c5 = self.min().z <= point.z;
        let c6 = self.max().z >= point.z;

        c1 && c2 && c3 && c4 && c5 && c6
    }
}