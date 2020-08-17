use std::io::BufReader;
use corale::wavefront::parse;
use std::fs::File;
use super::module::*;
use super::config::Config;

type Float = f64;

pub struct Wing {
    pub(crate) _angle: Float,
    pub(crate) modules: Vec<Module>,
}

impl Wing {
    pub(crate) fn new(index: usize, arc: Float, cfg: &Config) -> Self {
        let angle = index as Float * arc;
        let module_arc = arc / cfg.wings.module_count() as Float;
        let mut modules = Vec::new();
        
        let file = File::open(format!("assets/module.obj")).unwrap();
        let input = BufReader::new(file);
        let mesh = parse(input).unwrap();

        // Build the wing's modules
        for m in 0..cfg.wings.module_count() {
            let module = Module::new(3 * index + m, module_arc, &cfg, &mesh);
            modules.push(module);
        }
        Self {
            _angle: angle,
            modules,
        }
    }
}