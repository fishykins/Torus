use super::wing::Wing;
use crate::parse::parse_obj;
use std::fs::File;
use crate::export::*;
use std::io::BufReader;


pub struct Station {
    wings: Vec<Wing>,
}

impl Station {
    pub fn new(radius: f32, wing_count: usize, moudles_per_wing: usize) -> Self {
        // cache a few resued vars
        let wing_angle = (2. * std::f64::consts::PI as f32) / wing_count as f32;
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
            for m in &w.modules {
                let mesh = m.build();
                export_obj(mesh, format!("module_{}", m.index()));
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