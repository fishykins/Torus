use prima::core::GeoNum;
use crate::geom::{Torus};
use crate::config::*;
use super::Sector;
use super::Module;
use vek::Vec3;
use std::f64;

pub struct Station<T> where T: GeoNum {
    pub(crate) torus: Torus<T>,
    pub(crate) _config: Config,
    pub(crate) modules: Vec<Module<T>>,
}


impl<'a, T> Station<T> where T: GeoNum {
    pub fn new(config: Config) -> Self {
        let major: T = config.station.major().unwrap();
        let minor: T = config.station.minor().unwrap();
        let module_count = config.sectors.modules();
        let sector_count = config.station.sectors();
        let ang_incr: f64 = (2. * f64::consts::PI) / (sector_count as f64);

        let torus = Torus::new(major, minor, Vec3::zero());
        let mut modules = Vec::<Module<T>>::new();

        for i in 0..sector_count {
            let sector = Sector::<T>::new(i, ang_incr);
            for (j, arc) in sector.arc().subdivide(module_count).iter().enumerate() {
                let bbox = torus.make_arc_bbox(arc);
                let module = Module::new(j * sector.uid(), *arc, bbox);
                modules.push(module);
            }
        }

        Self {
            torus,
            modules,
            _config: config,
        }
    }

    pub fn torus(&self) -> &Torus<T> {
        &self.torus
    }

    pub fn module(&self, i: usize) -> &Module<T> {
        &self.modules[i]
    }
}

#[test]
fn station_test() {
    let cfg = Config::import("assets/World.toml");
    let _station = Station::<f64>::new(cfg);
}