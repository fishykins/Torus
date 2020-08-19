use corale::core::GeoNum;
use corale::core::maths;
use vek::Vec2;

/// represents a boundingbox, consisting of a segment of torus. Used to aid the conversion of an MPos into world-space
pub struct Arc<T> where T: GeoNum {
    a: T,
    b: T,
    size: Vec2<T>,
}

impl<T> Arc<T> where T: GeoNum {
    pub fn new(a: T, b: T, size: Vec2<T>) -> Self {
        Self {
            a,
            b,
            size,
        }
    }

    pub fn scope(&self) -> Vec2<T> {
        Vec2::new(self.a, self.b)
    }

    pub fn size(&self) -> Vec2<T> {
        self.size
    }

    /// lerps between angles a and b
    pub fn lerp(&self, theta: T) -> T {
        maths::lerp(self.a, self.b, theta)
    }

    /// lerp + clamp between angles a and b
    pub fn lerpc(&self, theta: T) -> T {
        maths::lerpc(self.a, self.b, theta)
    }
}