use std::io::BufReader;
use crate::parse::parse_obj;
use std::fs::File;
use super::module::*;

pub struct Wing {
    pub(crate) angle: f32,
    pub(crate) modules: Vec<Module>,
}

impl Wing {
    pub(crate) fn new(index: usize, radius: f32, module_count: usize, arc: f32) -> Self {
        let angle = index as f32 * arc;
        let module_arc = arc / module_count as f32;
        let mut modules = Vec::new();
        
        let file = File::open(format!("assets/module.obj")).unwrap();
        let input = BufReader::new(file);
        let mesh = parse_obj(input).unwrap();

        // Build the wing's modules
        for m in 0..module_count {
            let module = Module::new(3 * index + m, module_arc, radius, &mesh);
            modules.push(module);
        }
        Self {
            angle,
            modules,
        }
    }
}