use prima::core::GeoNum;
use prima::core::maths;
use vek::Vec2;
use super::{TPos, TBounds};

#[derive(Clone, Copy)]
/// represents a boundingbox, consisting of a segment of torus. Measured using a mixture of vectors and angles
pub struct Arc<T> where T: GeoNum {
    a: T, /// min angle of segment
    b: T, /// max angle of segment
    width: T,
    height: T,
    bounds: TBounds<T>,
}

impl<T> Arc<T> where T: GeoNum {
    /// generates new arc between a and b
    pub fn from_scope(a: T, b: T, width: T, height: T) -> Self {
        let two = T::one() + T::one();
        // x has its origin at the center of the arc, so width is split left and right
        // y has its origin at the "floor" of the arc, so min is 0 and max is height
        let min = TPos::new(- width / two, T::zero(), a); 
        let max = TPos::new(width / two, height, b); 
        let bounds = TBounds::new(min, max);

        Self {
            a,
            b,
            width,
            height,
            bounds,
        }
    }

    /// generates new arc with its center at theta
    pub fn new(theta: T, size: T, width: T, height: T) -> Self {
        let two = T::one() + T::one();
        let a = theta - size / two;
        let b = theta + size / two;
        Self::from_scope(a, b, width, height)
    }

    pub fn scope(&self) -> (T, T) {
        (self.a, self.b)
    }

    pub fn arc(&self) -> T {
        self.b - self.a
    }

    pub fn circ(&self) -> T {
        self.arc() / T::rad()
    }

    pub fn size(&self) -> Vec2<T> {
        Vec2::new(self.width, self.height)
    }

    /// lerps between angles a and b
    pub fn lerp(&self, theta: T) -> T {
        maths::lerp(self.a, self.b, theta)
    }

    /// lerp + clamp between angles a and b
    pub fn lerpc(&self, theta: T) -> T {
        maths::lerpc(self.a, self.b, theta)
    }

    /// gets the flat dimensions of the given arc
    pub fn bounds(&self) -> &TBounds<T> {
        &self.bounds
    }

    pub fn subdivide(&self, parts: usize) -> Vec<Self> {
        let mut subs = Vec::new();
        if parts <= 1 {
            panic!("Must be divided by at least 2!");
        }

        let node_arc_size = self.arc() / T::from_usize(parts).unwrap();

        for i in 0..parts {
            let n1 = T::from_usize(i).unwrap() * node_arc_size;
            let n2 = T::from_usize(i + 1).unwrap() * node_arc_size;
            subs.push(Arc::from_scope(n1, n2, self.width, self.height));
        }
        subs
    }
}

#[test]
fn subdive_test() {
    let arc = Arc::new(0., 360., 16., 16.);
    let arcs = arc.subdivide(3);
    assert_eq!(arcs.len(), 3);
}