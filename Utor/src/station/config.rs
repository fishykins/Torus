use vek::{Vec2, Vec3};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub station: StationConfig,
    pub wings: WingConfig,
    pub modules: ModuleConfig,
    pub rooms: RoomConfig,
    pub humans: HumanConfig,
}

#[derive(Deserialize, Clone)]
pub struct StationConfig {
    wings: usize,
    radius: f64,
}

#[derive(Deserialize, Clone)]
pub struct WingConfig {
    modules: usize,
}

#[derive(Deserialize, Clone)]
pub struct ModuleConfig {
    /// Number of rooms to aim for
    rooms: usize,
    /// Width & height of the module in proportion to the length (which is calculated based on numerous factors)
    width: f64,
    height: f64,
    inner_space: [f64; 3],
}

#[derive(Deserialize, Clone)]
pub struct RoomConfig {
    min_width: usize,
}

#[derive(Deserialize, Clone)]
pub struct HumanConfig {
    height: f64,
    width: f64,
}

impl Config {
    pub fn import(file: &str) -> Self {
        let file_string = fs::read_to_string(file).unwrap();
        let parsed: Self = toml::from_str(&file_string).unwrap();
        parsed
    }
}

impl StationConfig {
    pub fn wing_count(&self) -> usize {
        self.wings
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl WingConfig {
    pub fn module_count(&self) -> usize {
        self.modules
    }
}

impl ModuleConfig {
    pub fn room_count(&self) -> usize {
        self.rooms
    }
    pub fn dimensions(&self) -> Vec2<f64> {
        Vec2::new(self.width, self.height)
    }

    pub fn inner_space(&self) -> Vec3<f64> {
        Vec3::new(self.inner_space[0], self.inner_space[1], self.inner_space[2])
    }
}

impl RoomConfig {
    pub fn min_width(&self) -> usize {
        self.min_width
    }
}

impl HumanConfig {
    pub fn dimensions(&self) -> Vec2<f64> {
        Vec2::new(self.width, self.height)
    }
}


#[test]
fn station_config_test() {
    let cnf = Config::import("assets/config.toml");
    assert_eq!(20, cnf.modules.rooms);
}