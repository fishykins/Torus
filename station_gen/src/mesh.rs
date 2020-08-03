use vek::Vec3;

pub type Vertex = Vec3<f32>;
pub type Face = Vec<usize>;

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

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
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

    /// Getter for verts
    pub fn verticies(&self) -> &Vec<Vertex> {
        &self.verticies
    }

    /// getter for faces. Duh
    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }
}