use vek::Vec3;

pub trait Grid<T: PartialEq> {
    fn in_bounds(&self, pos: Vec3<usize>) -> bool;
    fn add(&mut self, item: T, pos: Vec3<usize>);
    fn item(&self, pos: Vec3<usize>) -> Option<&T>;
    fn position(&self, item: &T) -> Option<Vec3<usize>>;
    fn items(&self) -> Vec<(&T, Vec3<usize>)>;
    fn len(&self) -> usize;
}

#[derive(Clone)]
pub struct UGrid3<T> {
    grid_array: Vec<Vec<Vec<Option<usize>>>>,
    items: Vec<(T, Vec3<usize>)>,
    bounds: Vec3<usize>,
}

impl<T: PartialEq> UGrid3<T> {
    pub fn new(size: Vec3<usize>) -> Self {
        Self {
            bounds: size,
            items: Vec::new(),
            grid_array: vec![vec![vec![None; size.x]; size.y]; size.z],
        }
    }
}

impl<T: PartialEq> Grid<T> for UGrid3<T> {
    fn add(&mut self, item: T, pos: Vec3<usize>) {
        let i = self.items.len();
        self.items.push((item, pos));
        self.grid_array[pos.z][pos.y][pos.x] = Some(i);
    }

    fn item(&self, pos: Vec3<usize>) -> Option<&T> {
        if !self.in_bounds(pos) {
            return None;
        }

        let index = self.grid_array[pos.z][pos.y][pos.x];
        if index.is_some() {
            return Some(&self.items[index.unwrap()].0)
        } 

        return None
    }

    fn position(&self, item: &T) -> Option<Vec3<usize>> {
        let index = self.items.iter().position(|i| &i.0 == item);
        if index.is_some() {
            return Some(self.items[index.unwrap()].1);
        }
        None
    }

    fn in_bounds(&self, pos: Vec3<usize>) -> bool {
        pos.x <= self.bounds.x && pos.y <= self.bounds.y && pos.z <= self.bounds.z
    }

    fn items(&self) -> Vec<(&T, Vec3<usize>)> {
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