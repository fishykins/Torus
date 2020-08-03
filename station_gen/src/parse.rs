use std::io::BufRead;
use crate::mesh::*;
use crate::lexer::lex;
use crate::error::ObjResult;
use vek::Vec3;

macro_rules! parse_args {
    {
        $first:expr, $rest:expr,
        $($pat:pat => $type:ident::$name:ident[$exp:expr]),*,
        ! => $error:expr
    } => (
        match split_vertex_group($first)[..] {
            $($pat => $type::$name({
                let mut points = vec![$exp];
                for param in $rest {
                    match split_vertex_group(param)[..] {
                        $pat => points.push($exp),
                        _ => $error
                    }
                }
                points
            }),)*
            _ => $error
        }
    )
}

fn parse_args(args: &[&str]) -> Result<Vec<f32>, std::num::ParseFloatError> {
    args.iter().map(|arg| arg.parse()).collect()
}

/// Splits a string with '/'.
fn split_vertex_group(input: &str) -> Vec<&str> {
    input.split('/').collect()
}

/// A simple geometry parser for .obj fiels.
pub fn parse_obj<T: BufRead>(input: T) -> ObjResult<Mesh> {
    let mut mesh = Mesh::new();

    lex(input, |stmt, args: &[&str]| {
        match stmt {
            "v" => {
                mesh.add_vertex(match parse_args(args)?[..] {
                [x, y, z, _] => Vec3::new(x, z, y),
                [x, y, z] => Vec3::new(x, z, y),
                _ => make_error!(WrongNumberOfArguments, "Expected 3 or 4 arguments"),
                });
                Ok(())
            },
            "fo" | "f" => match args {
                [] => make_error!(WrongNumberOfArguments, "Expected at least 3 arguments"),
                [first, rest @ ..] => {
                    if args.len() < 3 {
                        make_error!(WrongNumberOfArguments, "Expected at least 3 arguments")
                    }

                    let polygon = parse_args! {
                        first, rest,
                        [p] => Polygon::P[try_index(&mesh.verticies, p)?],
                        [p, t] => Polygon::PT[(try_index(&mesh.verticies, p)?, try_index(&mesh.tex_coords, t)?)],
                        [p, "", n] => Polygon::PN[(try_index(&mesh.verticies, p)?, try_index(&mesh.normals, n)?)],
                        [p, t, n] => Polygon::PTN[(try_index(&mesh.verticies, p)?, try_index(&mesh.tex_coords, t)?, try_index(&mesh.normals, n)?)],
                        ! => make_error!(WrongTypeOfArguments, "Unexpected vertex format, expected `#`, `#/#`, `#//#`, or `#/#/#`")
                    };

                    // for the moment, deal with pure positional data
                    polygon.export_to_mesh(&mut mesh);

                    Ok(())
                }
            },
            "o" => {
                mesh.set_name(match args {
                    [] => None,
                    _ => Some(args.join(" ")),
                });
                Ok(())
            }

            //Things to just ignore
            "mtllib" | "#" => Ok(()),
            
            _ => Ok(()), //make_error!(UnexpectedStatement, "Type not found"),
        }
    })?;

    Ok(mesh)
}

/// The `Polygon` type.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Polygon {
    /// A polygon which contains only the position data of each vertex.
    P(Vec<usize>),
    /// A polygon which contains both position and texture coordinate data of each vertex.
    PT(Vec<(usize, usize)>),
    /// A polygon which contains both position and normal data of each vertex.
    PN(Vec<(usize, usize)>),
    /// A polygon which contains all position, texture coordinate and normal data of each vertex.
    PTN(Vec<(usize, usize, usize)>),
}

impl Polygon {
    fn export_to_mesh(self, mesh: &mut Mesh) {
        match self {
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
}

// Helper function for handling the indexes.
//
// If total size of the collection is 5:
//
// - ["1", "2", "3", "4", "5"] → [0, 1, 2, 3, 4]
// - ["-5", "-4", "-3", "-2", "-1"] → [0, 1, 2, 3, 4]
// - ["0"] → Error
// - ["6"] → Error
// - ["-6"] → Error
//
// If the index is < 0, then it represents an offset from the end of
// the current list. So -1 is the most recently added vertex.
//
// If the index is > 0 then it's simply the position in the list such
// that 1 is the first vertex.
fn try_index<T>(collection: &[T], input: &str) -> ObjResult<usize> {
    use crate::error::{LoadError, LoadErrorKind, ObjError};
    use std::convert::TryInto;

    let len: isize = collection.len().try_into().map_err(|_| {
        ObjError::Load(LoadError::new(
            LoadErrorKind::IndexOutOfRange,
            "Too many items in collection",
        ))
    })?;

    // Should be [-len, -1] ∪ [1, len]
    let index: isize = input.parse()?;

    let ret = if index < -len {
        // (∞, -len)
        make_error!(IndexOutOfRange, "Too small index value");
    } else if index < 0 {
        // [-len, 0)
        len + index
    } else if index == 0 {
        // {0}
        make_error!(IndexOutOfRange, "Index value shouldn't be zero");
    } else if index <= len {
        // (0, len]
        index - 1
    } else {
        // (len, ∞)
        make_error!(IndexOutOfRange, "Too big index value");
    };

    Ok(ret as usize)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::parse::parse_obj;
    use vek::Vec3;

    #[test]
    fn parse_test() {

        let file = File::open(format!("tests/objects/cube.obj")).unwrap();
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
        assert_eq!(mesh.verticies()[7], Vec3::new(-10., 10., -10.), "Vert data is wrong");
    }
}
