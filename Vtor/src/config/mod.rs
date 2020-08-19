mod station_cfg;

pub use station_cfg::StationCfg;

use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub station: StationCfg,
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