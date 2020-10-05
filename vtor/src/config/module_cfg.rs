use vek::Extent2;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ModuleCfg {
    pub seed: u64,
    pub room_count: usize,
    pub room_overflow: usize,
    pub extent: [f32; 2],
    pub divide_area_chance: f32,
    pub divide_disparity_chance: f32,
    pub split_offset: f32,
    pub split_degredation: f32,
}

impl ModuleCfg {
    pub fn new(seed: u64, room_count: usize) -> Self {
        Self {
            seed,
            room_count,
            room_overflow: (room_count as f32 / 2.) as usize,
            extent: [64., 128.],
            divide_area_chance: 0.55,
            divide_disparity_chance: 0.1,
            split_offset: 0.4,
            split_degredation: 0.9,
        }
    }

    pub fn extent(&self) -> Extent2<f32> {
        Extent2::new(self.extent[0], self.extent[1])
    }
}