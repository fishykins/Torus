mod station_cfg;
mod sector_cfg;
mod module_cfg;
mod human_cfg;

pub use station_cfg::StationCfg;
pub use sector_cfg::SectorCfg;
pub use module_cfg::ModuleCfg;
pub use human_cfg::HumanCfg;

use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub station: StationCfg,
    pub sectors: SectorCfg,
    pub modules: ModuleCfg,
    pub humans: HumanCfg,
}


impl Config {
    pub fn import(file: &str) -> Self {
        let file_string = fs::read_to_string(file).unwrap();
        let parsed: Self = toml::from_str(&file_string).unwrap();
        parsed
    }
}


#[test]
fn config_test() {
    Config::import("assets/World.toml");
}