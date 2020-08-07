pub mod mesh;
pub mod export;
pub mod parse;
pub mod primatives;

pub mod station;

#[cfg(test)]
mod tests {
    use crate::primatives::*;
    use crate::export::*;

    #[test]
    fn build_obj() {
        let mesh = default_cube(10.);
        mesh.set_name("cube");
        export_obj(mesh, "test_object").unwrap();
    }
}
