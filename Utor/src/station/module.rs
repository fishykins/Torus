use vek::{Vec2, Vec3};
use corale::mesh::*;
use corale::geom::*;

use crate::filters::TorusModifier;
use super::config::Config;
use super::generation::*;

type Float = f64;

/// main container for each segment of the ring. Handles mesh conversion
pub struct Module {
    index: usize,
    torus_mod: TorusModifier<Float>,
    bounds: BoundingBox<Float>, 
    mesh: Mesh<Float>,
}


impl Module {
    /// Creates a new module. 
    /// theta: the angle size of the segment's arc
    /// radius: the radius from the ring center to the middle of the module
    /// mesh: parent mesh to clone from. 
    pub fn new(index: usize, arc: Float, cfg: &Config, mesh: &Mesh<Float>) -> Self {
        let angle = index as Float * arc;
        let mut torus_mod = TorusModifier::new(Vec2::zero(), angle, cfg.station.radius(), arc, 0., 0.);
        let length = torus_mod.length();
        torus_mod.set_size(length * cfg.modules.dimensions().x, length * cfg.modules.dimensions().y);
        let mut new_mesh = mesh.clone();
        let name = format!("module_{}", index);
        new_mesh.set_name(name);

        // Calculate the dimensions of inahabitable space
        let bounds_max = Vec3::new(torus_mod.width(), torus_mod.height(), torus_mod.length());
        let mid_point = bounds_max / 2.;

        let offset = Vec3::new(
            torus_mod.width() * cfg.modules.inner_space().x * 0.5, 
            torus_mod.height() * cfg.modules.inner_space().y * 0.5, 
            torus_mod.length() * cfg.modules.inner_space().z * 0.5, 
        );

        let bounds = BoundingBox::new(mid_point - offset, mid_point + offset);

        println!("Bounds = {}m x {}m x {}m", bounds.width().floor(), bounds.height().floor(), bounds.depth().floor());

        build(&bounds, &cfg);

        Self {
            index,
            torus_mod,
            bounds,
            mesh: new_mesh,
        }
    }

    pub fn mesh(&self) -> &Mesh<Float> {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh<Float> {
        &mut self.mesh
    }

    pub fn bounds(&self) -> &BoundingBox<Float> {
        &self.bounds
    }

    pub fn build(&self) -> Mesh<Float> {
        let mut mesh = self.mesh.clone();
        self.torus_mod.apply(&mut mesh);
        mesh.invert_z();
        mesh
    }

    pub fn index(&self) -> usize {
        self.index
    }

}


#[cfg(test)]
mod tests {
    use crate::station::module::Module;
    use crate::station::config::*;
    use corale::mesh::Mesh;
    use corale::wavefront::*;
    use std::io::BufReader;
    use std::fs::File;
    
    #[test]
    fn module_test() {
        let cfg = Config::import("assets/config.toml");
        let angle = 2. * std::f64::consts::PI / cfg.station.wing_count() as f64 / cfg.wings.module_count()  as f64;
        
        let file = File::open(format!("assets/module.obj")).unwrap();
        let input = BufReader::new(file);
        let mesh: Mesh<f64> = parse(input).unwrap();
        
        let module = Module::new(0, angle, &cfg, &mesh);
        let build = module.build();
        let file_name = "../bin/renders/a_test".to_string();
        export(&build, file_name).unwrap();
    }
}