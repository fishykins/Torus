//#[macro_use]
//mod error;
pub mod mesh;
pub mod export;
pub mod parse;
pub mod primatives;

pub mod station;
//pub mod lexer;


#[cfg(test)]
mod tests {
    use crate::primatives::*;
    use crate::export::*;

    #[test]
    fn build_obj() {
        let mesh = default_cube(10.);
        export_obj(mesh, "cube", "test_object").unwrap();
    }
}
