use vek::{Vec2, Vec3};
use crate::mesh::*;

pub mod module;
pub mod cplane;
pub mod ring_space;

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
        let width = self.rm;

        let _cp = Vec2::new(r.1 * self.angle.cos(), r.1 * self.angle.sin());
        
        // ================= Station Inner ===================== //
        // Floor
        let p1 = Vec2::new(r.1 * angle_min.cos(), r.1 * angle_min.sin());
        let p2 = Vec2::new(r.1 * angle_max.cos(), r.1 * angle_max.sin());

        let dist = p1.distance(p2);
        let loop_width = 3.2; // Target distance between loop-cuts
        let edge_loop_count = (dist / loop_width).ceil() as u8;
        let edge_arc = self.arc / edge_loop_count as f32;

        // Make the first edge
        let v1 = mesh.add_vertex(Vec3::new(p1.x, p1.y, -width));
        let v2 = mesh.add_vertex(Vec3::new(p1.x, p1.y, width));
        let v3: usize;
        let v4: usize;
        let mut l1 = v1;
        let mut l2 = v2;
        let mut l3 = 0;
        let mut l4 = 0;

        for i in 0..edge_loop_count+1 {
            let angle = angle_min + edge_arc * i as f32;
            let target = Vec2::new(r.1 * angle.cos(), r.1 * angle.sin());
            l3 = mesh.add_vertex(Vec3::new(target.x, target.y, -width));
            l4 = mesh.add_vertex(Vec3::new(target.x, target.y, width));

            mesh.add_face(vec![l3, l4, l2, l1]);
            // Move 3,4 -> 1,2
            l1 = l3;
            l2 = l4;
        }

        v3 = l3.clone();
        v4 = l4.clone();
        
        // Roof
        let roof1 = Vec2::new(r.0 * angle_min.cos(), r.0 * angle_min.sin());
        let roof2 = Vec2::new(r.0 * angle_max.cos(), r.0 * angle_max.sin());
        let v5 = mesh.add_vertex(Vec3::new(roof1.x, roof1.y, -width));
        let v6 = mesh.add_vertex(Vec3::new(roof1.x, roof1.y, width));
        let v7 = mesh.add_vertex(Vec3::new(roof2.x, roof2.y, -width));
        let v8 = mesh.add_vertex(Vec3::new(roof2.x, roof2.y, width));
        mesh.add_face(vec![v5, v6, v8, v7]);

        // Walls
        mesh.add_face(vec![v3, v7, v5, v1]);
        mesh.add_face(vec![v4, v8, v6, v2]);
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
        let mesh = build(1000., 64., 6, 4).unwrap();
        export_obj(mesh, "station", "station_test").unwrap();
    }
}
