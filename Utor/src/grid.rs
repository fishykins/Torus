use num::{Integer, Unsigned, ToPrimitive, FromPrimitive};
use vek::Vec3;
use crate::box_collider::BoxCollider;

pub trait Grid<I: PartialEq, T: Integer + Unsigned + ToPrimitive + FromPrimitive + Clone + Copy>: BoxCollider<T> {
    fn in_bounds(&self, pos: Vec3<T>) -> bool;
    fn add(&mut self, item: I, pos: Vec3<T>) -> bool;
    fn remove(&mut self, pos: Vec3<T>) -> bool;
    fn replace(&mut self, item: I, pos: Vec3<T>) -> bool;
    fn set(&mut self, item: I, pos: Vec3<T>) -> bool;
    fn item(&self, pos: Vec3<T>) -> Option<&I>;
    fn position(&self, item: &I) -> Option<Vec3<T>>;
    fn items(&self) -> Vec<(&I, Vec3<T>)>;
    fn len(&self) -> usize;
}

#[derive(Clone)]
/// Handles a 3D grid of points within an integer based system. 
pub struct Grid3<I: PartialEq, T: Integer + Unsigned + ToPrimitive + FromPrimitive + Clone + Copy> {
    grid_array: Vec<Vec<Vec<Option<usize>>>>,
    items: Vec<(I, Vec3<T>)>,
    bounds: Vec3<T>,
    removed: Vec<usize>,
}

impl<I: PartialEq, T: Integer + Unsigned + ToPrimitive + FromPrimitive + Clone + Copy> Grid3<I, T> {
    pub fn new(size: Vec3<T>) -> Self {
        Self {
            bounds: size,
            items: Vec::new(),
            grid_array: vec![vec![vec![None; size.x.to_usize().unwrap()]; size.y.to_usize().unwrap()]; size.z.to_usize().unwrap()],
            removed: Vec::new(),
        }
    }

    pub fn neighbors<F>(&self, pos: Vec3<T>, condition: F) -> Vec<(&I, Vec3<T>)>
        where F: for<'a> Fn(&'a I) -> bool
    {
        let min_x = if pos.x <= T::zero() { 0 } else {(pos.x - T::one()).to_usize().unwrap()};
        let max_x = (pos.x + T::one()).to_usize().unwrap();
        let min_y = if pos.y <= T::zero() { 0 } else {(pos.y - T::one()).to_usize().unwrap()};
        let max_y = (pos.y + T::one()).to_usize().unwrap();
        let min_z = if pos.z <= T::zero() { 0 } else {(pos.z - T::one()).to_usize().unwrap()};
        let max_z = (pos.z + T::one()).to_usize().unwrap();

        let mut array = Vec::new();

        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                for z in min_z..max_z + 1 {
                    let p = Vec3::new(x,y,z).map(|i| T::from_usize(i).unwrap());
                    if p == pos {
                        continue;
                    }

                    let item = self.item(p);
                    if item.is_some() {
                        if condition(item.unwrap()) {
                            array.push((item.unwrap(), p));
                        }
                    }
                }
            }    
        }
        array
    }

    pub fn cross_neighbors<F>(&self, pos: Vec3<T>, condition: F) -> Vec<(&I, Vec3<T>)>
        where F: for<'a> Fn(&'a I) -> bool
    {
        let mut neighbors = vec![ 
            Vec3::new(pos.x + T::one(), pos.y, pos.z),
            Vec3::new(pos.x, pos.y + T::one(), pos.z),
            Vec3::new(pos.x, pos.y, pos.z + T::one()),
        ];

        if pos.x >= T::one() { neighbors.push(Vec3::new(pos.x - T::one(), pos.y, pos.z))};
        if pos.y >= T::one() { neighbors.push(Vec3::new(pos.x, pos.y - T::one(), pos.z))};
        if pos.z >= T::one() { neighbors.push(Vec3::new(pos.x, pos.y, pos.z - T::one()))};

        let mut array = Vec::new();

        for n in neighbors {
            let item = self.item(n);
            if item.is_some() {
                if condition(item.unwrap()) {
                    array.push((item.unwrap(), n));
                }
            }
        }
        array
    }

    fn index(&self, pos: Vec3<T>) -> Option<usize> {
        let x = pos.x.to_usize().unwrap();
        let y = pos.y.to_usize().unwrap();
        let z = pos.z.to_usize().unwrap();
        if pos.x >= self.bounds.x || pos.y >= self.bounds.y || pos.z >= self.bounds.z {
            return None;
        }
        self.grid_array[z][y][x]
    }

    pub fn clone_empty(&self) -> Self {
        Self {
            bounds: self.bounds.clone(),
            items: Vec::new(),
            grid_array: vec![vec![vec![None; self.bounds.x.to_usize().unwrap()]; self.bounds.y.to_usize().unwrap()]; self.bounds.z.to_usize().unwrap()],
            removed: Vec::new(),
        }
    }
}


impl<I: PartialEq, T: Integer + Unsigned + ToPrimitive + FromPrimitive + Clone + Copy> BoxCollider<T> for Grid3<I, T> {
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

impl<I: PartialEq, T: Integer + Unsigned + ToPrimitive + FromPrimitive + Clone + Copy> Grid<I, T> for Grid3<I, T> {
    fn add(&mut self, item: I, pos: Vec3<T>) -> bool {
        
        if pos.x >= self.bounds.x || pos.y >= self.bounds.y || pos.z >= self.bounds.z {
            return false;
        }
        let i = self.items.len();
        self.items.push((item, pos));
        self.grid_array[pos.z.to_usize().unwrap()][pos.y.to_usize().unwrap()][pos.x.to_usize().unwrap()] = Some(i);
        true
    }

    fn set(&mut self, item: I, pos: Vec3<T>) -> bool {
        if self.index(pos).is_some() {
            return self.replace(item, pos)
        } else {
            return self.add(item, pos)
        }
    }

    fn remove(&mut self, pos: Vec3<T>) -> bool {
        if let Some(index) = self.index(pos) {
            self.removed.push(index);
            self.grid_array[pos.z.to_usize().unwrap()][pos.y.to_usize().unwrap()][pos.x.to_usize().unwrap()] = None;
            return true;
        } 
        return false;
    }

    fn replace(&mut self, item: I, pos: Vec3<T>) -> bool {
        if let Some(index) = self.index(pos) {
            self.items[index] = (item, pos);
            return true;
        }
        false
    }

    fn item(&self, pos: Vec3<T>) -> Option<&I> {
        if !self.in_bounds(pos) {
            return None;
        }
        if let Some(index) = self.index(pos) {
            return Some(&self.items[index].0)
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
        for (i, item) in self.items.iter().enumerate() {
            if !self.removed.contains(&i) {
                array.push((&item.0, item.1));
            }
        }
        array
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::*;
    
    #[test]
    fn grid_test() {
        let mut grid = Grid3::<bool, u32>::new(Vec3::from(16));
        assert!(grid.add(true, Vec3::from(0)));
        assert!(grid.add(true, Vec3::from(3)));
        assert!(grid.add(true, Vec3::from(4)));
        assert!(grid.add(false, Vec3::from(5)));
        assert!(!grid.add(false, Vec3::from(16)));

        assert_eq!(grid.items().len(), 4);
        assert!(grid.item(Vec3::from(0)).is_some());
        assert_eq!(grid.neighbors(Vec3::from(4), |x| *x).len(), 1);
        assert!(grid.item(Vec3::from(5)).is_some());
        assert!(grid.remove(Vec3::from(5)));
        assert!(grid.item(Vec3::from(5)).is_none());
    }
}