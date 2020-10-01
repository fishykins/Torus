use serde::Deserialize;
use prima::core::OrdNum;

#[derive(Deserialize, Clone)]
pub struct StationCfg {
    sectors: usize,
    major: f64,
    minor: f64,
}

impl StationCfg {
    pub fn sectors(&self) -> usize {
        self.sectors
    }

    pub fn major<T>(&self) -> Option<T> where T: OrdNum {
        T::from_f64(self.major)
    }

    pub fn minor<T>(&self) -> Option<T> where T: OrdNum {
        T::from_f64(self.minor)
    }
}