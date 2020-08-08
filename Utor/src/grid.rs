use num::{Integer, ToPrimitive};
use vek::Vec3;
use crate::box_collider::BoxCollider;

pub trait Grid<I: PartialEq, T: Integer + ToPrimitive + Clone + Copy>: BoxCollider<T> {
    fn in_bounds(&self, pos: Vec3<T>) -> bool;
    fn add(&mut self, item: I, pos: Vec3<T>);
    fn item(&self, pos: Vec3<T>) -> Option<&I>;
    fn position(&self, item: &I) -> Option<Vec3<T>>;
    fn items(&self) -> Vec<(&I, Vec3<T>)>;
    fn len(&self) -> usize;
}

#[derive(Clone)]
/// Handles a 3D grid of points within an integer based system. 
pub struct Grid3<I: PartialEq, T: Integer + ToPrimitive + Clone + Copy> {
    grid_array: Vec<Vec<Vec<Option<usize>>>>,
    items: Vec<(I, Vec3<T>)>,
    bounds: Vec3<T>,
}

impl<I: PartialEq, T: Integer + ToPrimitive + Clone + Copy> Grid3<I, T> {
    pub fn new(size: Vec3<T>) -> Self {
        Self {
            bounds: size,
            items: Vec::new(),
            grid_array: vec![vec![vec![None; size.x.to_usize().unwrap()]; size.y.to_usize().unwrap()]; size.z.to_usize().unwrap()],
        }
    }
}


impl<I: PartialEq, T: Integer + ToPrimitive + Clone + Copy> BoxCollider<T> for Grid3<I, T> {
    fn min(&self) -> Vec3<T> {
        Vec3::zero()
    }

    fn max(&self) -> Vec3<T> {
        self.bounds.clone()
    }

    fn contains(&self, other: &dyn BoxCollider<T>) -> bool {
        let c1 = self.min().x <= other.min().x;
        let c2 = self.max().x >= other.max().x;
        let c3 = self.min().y <= other.min().y;
        let c4 = self.max().y >= other.max().y;
        let c5 = self.min().z <= other.min().z;
        let c6 = self.max().z >= other.max().z;

        c1 && c2 && c3 && c4 && c5 && c6
    }

    fn intersects(&self, other: &dyn BoxCollider<T>) -> bool {
        let c1 = self.min().x > other.max().x;
        let c2 = self.max().x < other.min().x;
        let c3 = self.min().y > other.max().y;
        let c4 = self.max().y < other.min().y;
        let c5 = self.min().z > other.max().z;
        let c6 = self.max().z < other.min().z;

        !c1 && !c2 && !c3 && !c4 && !c5 && !c6
    }
}

impl<I: PartialEq, T: Integer + ToPrimitive + Clone + Copy> Grid<I, T> for Grid3<I, T> {
    fn add(&mut self, item: I, pos: Vec3<T>) {
        let i = self.items.len();
        self.items.push((item, pos));
        self.grid_array[pos.z.to_usize().unwrap()][pos.y.to_usize().unwrap()][pos.x.to_usize().unwrap()] = Some(i);
    }

    fn item(&self, pos: Vec3<T>) -> Option<&I> {
        if !self.in_bounds(pos) {
            return None;
        }

        let index = self.grid_array[pos.z.to_usize().unwrap()][pos.y.to_usize().unwrap()][pos.x.to_usize().unwrap()];
        if index.is_some() {
            return Some(&self.items[index.unwrap()].0)
        } 

        return None
    }

    fn position(&self, item: &I) -> Option<Vec3<T>> {
        let index = self.items.iter().position(|i| &i.0 == item);
        if index.is_some() {
            return Some(self.items[index.unwrap()].1.clone());
        }
        None
    }

    fn in_bounds(&self, pos: Vec3<T>) -> bool {
        pos.x <= self.bounds.x && pos.y <= self.bounds.y && pos.z <= self.bounds.z
    }

    fn items(&self) -> Vec<(&I, Vec3<T>)> {
        let mut array = Vec::new();
        for i in &self.items {
            array.push((&i.0, i.1));
        }
        array
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}