use corale::core::GeoNum;
use corale::core::maths;
use vek::Vec2;
use super::{TPos, TBounds};

/// represents a boundingbox, consisting of a segment of torus. Measured using a mixture of vectors and angles
pub struct Arc<T> where T: GeoNum {
    a: T, /// min angle of segment
    b: T, /// max angle of segment
    width: T,
    height: T,
    bounds: TBounds<T>,
}

impl<T> Arc<T> where T: GeoNum {
    pub fn new(a: T, b: T, width: T, height: T) -> Self {
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

    pub fn scope(&self) -> (T, T) {
        (self.a, self.b)
    }

    pub fn arc(&self) -> T {
        self.b - self.a
    }

    pub fn length(&self, radius: T) -> T {
        let pi2 = T::from_f64(std::f64::consts::PI * 2.).unwrap();
        (self.arc() / pi2) * radius
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
}