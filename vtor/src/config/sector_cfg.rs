use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SectorCfg {
    modules: usize,
}

impl SectorCfg {
    pub fn modules(&self) -> usize {
        self.modules
    }
}