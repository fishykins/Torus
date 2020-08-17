use super::wing::Wing;
use super::config::*;
use corale::wavefront::export;

type Float = f64;

pub struct Station {
    wings: Vec<Wing>,
}

impl Station {
    pub fn new(cfg: Config) -> Self {
        // cache a few resued vars
        let wing_angle = (2. * std::f64::consts::PI as Float) / cfg.station.wing_count() as Float;
        let mut wings = Vec::new();

        // Build each wing of the station
        for i in 0..cfg.station.wing_count() {
            let wing = Wing::new(i, wing_angle, &cfg);
            wings.push(wing);
        }

        Self {
            wings,
        }
    }

    pub fn build(&self) {
        for w in &self.wings {
            for module in &w.modules {
                let mesh = module.build();
                let file_name = mesh.name().unwrap();
                export(&mesh, file_name).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::station::station::Station;
    use crate::station::config::Config;
    
    #[test]
    fn station_test() {
        let station = Station::new(Config::import("assets/config.toml"));
        station.build();
    }
}