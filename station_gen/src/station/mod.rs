use vek::{Vec2};
use crate::mesh::*;

#[derive(Debug)]
struct Spoke {
    a: Vec2<f32>,
    b: Vec2<f32>,
}

impl Spoke {
    fn _center(&self) -> Vec2<f32> {
        Vec2::new((self.a.x + self.b.x) / 2., (self.a.y + self.b.y) / 2. )
    }
}

/// Builds a segmented station. All units in meters
pub fn build(major_radius: f32, _minor_radius: f32, spoke_count: u8) -> Mesh {

    // cache a few resued vars
    let theta = (2. * std::f64::consts::PI as f32) / spoke_count as f32;
    let mut mesh = Mesh::new();

    // First get coordinates for spoke end-points
    let mut spokes = Vec::new();
    for i in 0..spoke_count {

        let angle = i as f32 * theta;
        let x = major_radius * angle.cos();
        let y = major_radius * angle.sin();

        let spoke = Spoke {
            a: Vec2::zero(),
            b: Vec2::new(x, y),
        };
        println!("{:?}", spoke);
        mesh.add_vertex(spoke.b.into());
        spokes.push(spoke);
    }

    mesh
}

#[cfg(test)]
mod tests {
    use crate::export::*;
    use crate::station::build;

    #[test]
    fn build_test() {
        let mesh = build(16., 4., 6);
        export_obj(mesh, "station", "station_test").unwrap();
    }
}
