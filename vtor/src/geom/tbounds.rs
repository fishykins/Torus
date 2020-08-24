use corale::core::GeoNum;
use super::TPos;

#[derive(Clone, Copy, Debug)]
pub struct TBounds<T> where T: GeoNum {
    min: TPos<T>,
    max: TPos<T>,
}

impl<T> TBounds<T> where T: GeoNum {
    pub fn new(min: TPos<T>, max: TPos<T>) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn min(&self) -> TPos<T> {
        self.min.clone()
    }
    pub fn max(&self) -> TPos<T> {
        self.max.clone()
    }

    pub fn contains(&self, other: &TBounds<T>) -> bool {
        let c1 = self.min().x <= other.min().x;
        let c2 = self.max().x >= other.max().x;
        let c3 = self.min().y <= other.min().y;
        let c4 = self.max().y >= other.max().y;
        let c5 = self.min().theta <= other.min().theta;
        let c6 = self.max().theta >= other.max().theta;

        c1 && c2 && c3 && c4 && c5 && c6
    }

    pub fn intersects(&self, other: &TBounds<T>) -> bool {
        let c1 = self.min().x > other.max().x;
        let c2 = self.max().x < other.min().x;
        let c3 = self.min().y > other.max().y;
        let c4 = self.max().y < other.min().y;
        let c5 = self.min().theta > other.max().theta;
        let c6 = self.max().theta < other.min().theta;

        !c1 && !c2 && !c3 && !c4 && !c5 && !c6
    }

    pub fn contains_point(&self, point: TPos<T>) -> bool {
        let c1 = self.min().x <= point.x;
        let c2 = self.max().x >= point.x;
        let c3 = self.min().y <= point.y;
        let c4 = self.max().y >= point.y;
        let c5 = self.min().theta <= point.theta;
        let c6 = self.max().theta >= point.theta;

        c1 && c2 && c3 && c4 && c5 && c6
    }
}