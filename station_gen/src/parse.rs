use crate::mesh::*;
use std::io::{Error, BufRead};
use obj::{raw::{parse_obj as parse, object::Polygon}};
use vek::Vec3;


pub fn parse_obj<T: BufRead>(input: T) -> Result<Mesh, Error> {
    let raw = parse(input).unwrap();

    // Convert to mesh, because RawObj = yucky
    let mut mesh = Mesh::new();
    for p in raw.positions {
        mesh.add_vertex(Vec3::new(p.0,p.2,p.1)); // Y and Z are swapped round, and we ignore W (p.3)
    }

    for p in raw.polygons {
        match p {
            Polygon::P(face) => {
                mesh.add_face(face);
            },
            Polygon::PT(face) => {
                mesh.add_face(face.iter().map(|x| x.0).collect());
            },
            Polygon::PN(face) => {
                mesh.add_face(face.iter().map(|x| x.0).collect());
            },
            Polygon::PTN(face) => {
                mesh.add_face(face.iter().map(|x| x.0).collect());
            }
        }
    }

    // Return the mesh
    Ok(mesh)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::parse::parse_obj;
    use vek::Vec3;

    #[test]
    fn parse_test() {

        let file = File::open(format!("assets/primatives/cube.obj")).unwrap();
        let input = BufReader::new(file);
        let mesh = parse_obj(input).unwrap();
        let mut index = 0;
        for i in mesh.verticies() {
            println!("v{}: {:?}", index, i);
            index += 1;
        }
        index = 0;
        for i in mesh.faces() {
            println!("f{}: {:?}", index, i);
            index += 1;
        }
        assert_eq!(mesh.faces()[2], [7, 6, 4, 5], "Face data is wrong");
        assert_eq!(mesh.verticies()[7], Vec3::new(-1., 1., -1.), "Vert data is wrong");
    }
}