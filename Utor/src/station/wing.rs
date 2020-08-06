use vek::Vec2;
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

        // Build the wing's modules
        for m in 0..module_count {
            let module = Module::new(3 * index + m, module_arc, radius);
            modules.push(module);
        }
        Self {
            angle,
            modules,
        }
    }
}