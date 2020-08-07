use super::wing::Wing;
use crate::export::*;

type Float = f64;

pub struct Station {
    wings: Vec<Wing>,
}

impl Station {
    pub fn new(radius: Float, wing_count: usize, moudles_per_wing: usize) -> Self {
        // cache a few resued vars
        let wing_angle = (2. * std::f64::consts::PI as Float) / wing_count as Float;
        let mut wings = Vec::new();

        // Build each wing of the station
        for i in 0..wing_count {
            let wing = Wing::new(i, radius, moudles_per_wing, wing_angle);
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
                export_obj(mesh, file_name).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::station::station::Station;
    
    #[test]
    fn station_test() {
        let station = Station::new(800., 6, 3);
        station.build();
    }
}