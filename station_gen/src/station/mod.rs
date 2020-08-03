use vek::{Vec2, Vec3};
use crate::mesh::*;

#[derive(Debug)]
struct Spoke {
    a: Vec2<f32>,
    b: Vec2<f32>,
}

struct Module {
    angle: f32,
    arc: f32,
    r: f32,
    rm: f32,
}

struct Wing {
    angle: f32,
    pos: Vec2<f32>,
    spoke: Spoke,
    modules: Vec<Module>,
}

struct Station {
    wings: Vec<Wing>,
}

impl Spoke {
    fn new(a: Vec2<f32>, b: Vec2<f32>) -> Self {
        Self {
            a,
            b,
        }
    }

    fn center(&self) -> Vec2<f32> {
        Vec2::new((self.a.x + self.b.x) / 2., (self.a.y + self.b.y) / 2. )
    }
}

impl Module {
    fn new(angle: f32,  arc: f32, r: f32, rm: f32) -> Self {
        Self {
            r,
            rm,
            angle,
            arc,
        }
    }

    fn build(&self, mesh: &mut Mesh) {
        let r = (self.r - self.rm, self.r + self.rm);
        let harc = self.arc / 2.;
        let angle_min = self.angle - harc;
        let angle_max = self.angle + harc;

        let cp = Vec2::new(r.1 * self.angle.cos(), r.1 * self.angle.sin());
        let p1 = Vec2::new(r.1 * angle_min.cos(), r.1 * angle_min.sin());
        let p2 = Vec2::new(r.1 * angle_max.cos(), r.1 * angle_max.sin());
        let p3 = Vec2::new(r.0 * angle_min.cos(), r.0 * angle_min.sin());
        let p4 = Vec2::new(r.0 * angle_max.cos(), r.0 * angle_max.sin());

        mesh.add_vertex(cp.into());
        let v1 = mesh.add_vertex(Vec3::new(p1.x, p1.y, -10.));
        let v2 = mesh.add_vertex(Vec3::new(p1.x, p1.y, 10.));
        let v3 = mesh.add_vertex(Vec3::new(p2.x, p2.y, -10.));
        let v4 = mesh.add_vertex(Vec3::new(p2.x, p2.y, 10.));

        let v5 = mesh.add_vertex(Vec3::new(p3.x, p3.y, -10.));
        let v6 = mesh.add_vertex(Vec3::new(p3.x, p3.y, 10.));
        let v7 = mesh.add_vertex(Vec3::new(p4.x, p4.y, -10.));
        let v8 = mesh.add_vertex(Vec3::new(p4.x, p4.y, 10.));

        mesh.add_face(vec![v1, v2, v4, v3]);
        mesh.add_face(vec![v5, v6, v8, v7]);
    }
}

impl Wing {
    fn new(angle: f32, r: f32) -> Self {
        let pos = Vec2::new(r * angle.cos(), r * angle.sin());
        Self {
            angle,
            pos,
            spoke: Spoke::new(Vec2::zero(), pos),
            modules: Vec::new(),
        }
    }
}

impl Station {
    fn new() -> Self {
        Self {
            wings: Vec::new(),
        }
    }

    fn build_mesh(&self) -> Result<Mesh, String> {
        let mut mesh = Mesh::new();
        for wing in self.wings.iter() {
            for module in wing.modules.iter() {
                module.build(&mut mesh);
            }
        }
        Ok(mesh)
    }
}

/// Builds a segmented station. All units in meters
pub fn build(major_radius: f32, minor_radius: f32, wing_count: u8, moudles_per_wing: u8) -> Result<Mesh, String> {

    // cache a few resued vars
    let t_wings = (2. * std::f64::consts::PI as f32) / wing_count as f32;
    let t_modules = t_wings / moudles_per_wing as f32;
    let mut station = Station::new();

    // Build each wing of the station
    for i in 0..wing_count {
        let mut wing = Wing::new(i as f32 * t_wings, major_radius);
        // Build the wing's modules
        let start_theta = wing.angle - (t_wings / 2.);
        for m in 0..moudles_per_wing {
            let module = Module::new(start_theta + (t_modules * m as f32), t_modules, major_radius, minor_radius);
            wing.modules.push(module);
        }
        station.wings.push(wing);
    }

    station.build_mesh()
}

#[cfg(test)]
mod tests {
    use crate::export::*;
    use crate::station::build;

    #[test]
    fn build_test() {
        let mesh = build(16., 4., 6, 4).unwrap();
        export_obj(mesh, "station", "station_test").unwrap();
    }
}
