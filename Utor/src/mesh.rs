pub use vek::Vec3;

pub type Vertex = Vec3<f32>;
pub type Face = Vec<usize>;

#[derive(Clone)]
#[no_mangle]
pub struct Mesh {
    pub(crate) verticies: Vec<Vertex>,
    pub(crate) faces: Vec<Face>,
    pub(crate) _tex_coords: Vec<Vertex>,
    pub(crate) _normals: Vec<Vertex>,
    name: Option<String>,
}

impl Mesh {

    pub fn new() -> Self {
        Self {
            verticies: Vec::new(),
            faces: Vec::new(),
            _tex_coords: Vec::new(),
            _normals: Vec::new(),
            name: None,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.into());
    }

    /// Adds a lone vertex to the mesh. Be sure to give him some friends!
    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        let i = self.verticies.len();
        self.verticies.push(vertex);
        i
    }

    /// Adds a face to the mesh. Assumes the vertecies are already in the mesh
    pub fn add_face(&mut self, face: Face) -> usize {
        let i = self.faces.len();
        self.faces.push(face);
        i
    }

    /// Generates a face from given points and adds the vertecies to the mesh. Not to be used in conjunction with add_vertex or add_face
    pub fn make_face(&mut self, verticies: Vec<Vertex>) -> usize {
        let mut face = Vec::with_capacity(verticies.len());
        for i in 0..verticies.len() {
            let vi = self.add_vertex(verticies[i]);
            face[i] = vi;
        }
        self.add_face(face)
    }

    /// Normalizes the mesh using given Vec3
    pub fn normalize(&mut self, offset: Vec3<f32>) {
        self.map_verts(|v| v - offset);
    }

    pub fn invert_x(&mut self) {
        self.map_verts(|v| Vec3::new(-v.x, v.y, v.z));
    }

    pub fn invert_y(&mut self) {
        self.map_verts(|v| Vec3::new(v.x, -v.y, v.z));
    }

    pub fn invert_z(&mut self) {
        self.map_verts(|v| Vec3::new(v.x, v.y, -v.z));
    }

    /// Getter for verts
    pub fn verticies(&self) -> &Vec<Vertex> {
        &self.verticies
    }

    /// getter for faces. Duh
    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }

    pub fn name(&self) -> Option<String> {
        self.name
    }

    pub fn map_verts<F>(&mut self, f: F) where F: Fn(&Vertex) -> Vertex {
        self.verticies = self.verticies.iter().map(|x| f(&x)).collect();
    }

    #[no_mangle]
    pub extern fn mesh_double_input(&self, input: i32) -> i32 {
    input * 2
}
}