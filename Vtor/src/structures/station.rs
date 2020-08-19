use corale::core::GeoNum;
use crate::geom::Torus;
use crate::config::*;
use super::Module;
use vek::Vec3;

pub struct Station<'a, T> where T: GeoNum {
    torus: Torus<T>,
    config: Config,
    modules: Vec<Module<'a, T>>,
}


impl<'a, T> Station<'a, T> where T: GeoNum {
    pub fn new(config: Config) -> Self {
        let major: T = config.station.major().unwrap();
        let minor: T = config.station.minor().unwrap();
        Self {
            torus: Torus::new(major, minor, Vec3::zero()),
            modules: Vec::new(),
            config,
        }
    }

    pub fn add_module(&'a mut self) {
        let module = Module::new(&self.torus);
        self.modules.push(module);
    }
}

#[test]
fn station_test() {
    let cfg = Config::import("assets/World.toml");
    let mut station = Station::<f64>::new(cfg);
    station.add_module();
}