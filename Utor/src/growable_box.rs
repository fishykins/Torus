use crate::box_collider::BoxCollider;
use vek::Vec3;

#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
    None,
}

#[derive(PartialEq, Clone)]
pub struct GrowableBox {
    bounds: Vec3<(u32, u32)>,
}

impl GrowableBox {
    pub fn new(pos: Vec3<u32>) -> Self {
        Self {
            bounds: pos.zip(pos),
        }
    }

    pub fn pos(&self) -> Vec3<u32> {
        let min: Vec3<u32> = self.min().map(|z| z as u32);
        min + self.size()
    }

    pub fn size(&self) -> Vec3<u32> {
        let min: Vec3<u32> = self.min().map(|z| z as u32);
        let max: Vec3<u32> = self.max().map(|z| z as u32);
        max - min
        
    }

    pub fn expand(&mut self, direction: &Direction, amount: u8) {
        match direction {
            Direction::Front => self.bounds.z.1 += amount as u32,
            Direction::Back => self.bounds.z.0 -= amount as u32,
            Direction::Right => self.bounds.x.1 += amount as u32,
            Direction::Left => self.bounds.x.0 -= amount as u32,
            Direction::Up => self.bounds.y.1 += amount as u32,
            Direction::Down => self.bounds.y.0 -= amount as u32,
            _ => self.twiddle_thumbs(),
        };
    }

    fn twiddle_thumbs(&self) {

    }
}

impl BoxCollider<u32> for GrowableBox {
    fn min(&self) -> Vec3<u32> {
        self.bounds.map(|a| a.0)
    }

    fn max(&self) -> Vec3<u32> {
        self.bounds.map(|a| a.1)
    }

    fn contains(&self, other: &dyn BoxCollider<u32>) -> bool {
        let c1 = self.min().x <= other.min().x;
        let c2 = self.max().x >= other.max().x;
        let c3 = self.min().y <= other.min().y;
        let c4 = self.max().y >= other.max().y;
        let c5 = self.min().z <= other.min().z;
        let c6 = self.max().z >= other.max().z;

        c1 && c2 && c3 && c4 && c5 && c6
    }

    fn intersects(&self, other: &dyn BoxCollider<u32>) -> bool {
        let c1 = self.min().x > other.max().x;
        let c2 = self.max().x < other.min().x;
        let c3 = self.min().y > other.max().y;
        let c4 = self.max().y < other.min().y;
        let c5 = self.min().z > other.max().z;
        let c6 = self.max().z < other.min().z;

        !c1 && !c2 && !c3 && !c4 && !c5 && !c6
    }
}