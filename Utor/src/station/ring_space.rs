use vek::{Vec2, Vec3};
use crate::mesh::*;

type Float = f64;

fn clamp(min: Float, max: Float, value: Float) -> Float {
    if value > max {
        return max;
    } else if value < min {
        return min;
    }
    value
}

/// Lerp between a and b by amount (0 - 1)
fn lerp(a: Float, b: Float, amount: Float) -> Float {
    if a == b {
        return a;
    }
    if a > b {
        let range = a - b;
        return b + ( range * amount);
    } else {
        let range = b - a;
        return a + ( range * amount);
    }
}

/// Represents a single point in ring space. Each value must fall between 0 and 1
#[derive(Clone, Debug)]
pub struct RPos {
    arc: Float,
    y: Float,
    x: Float,
}

/// Represents a boundingbox that can translate between ring and world space. 
#[derive(Clone, Debug)]
pub struct RBox {
    origin: Vec2<Float>,
    angle: Float,
    arm: Float,
    arc: Float,
    height: Float,
    width: Float,
    length: Float,
}

impl RPos {
    pub fn new(x: Float, y: Float, arc: Float) -> Self {
        Self {
            arc: clamp(0., 1., arc),
            y: clamp(0., 1., y),
            x: clamp(0., 1., x),
        }
    }
}

impl From<Vec3<Float>> for RPos {
    fn from(v: Vec3<Float>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<&Vec3<Float>> for RPos {
    fn from(v: &Vec3<Float>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl RBox {
    pub fn new(origin: Vec2<Float>, angle: Float, arm: Float, arc: Float, height: Float, width: Float) -> Self {
        let theta = (- arc / 2., arc / 2.);
        let front = Vec2::new(arm * theta.1.cos(), arm * theta.1.sin());
        let back = Vec2::new(arm * theta.0.cos(), arm * theta.0.sin());
        Self {
            origin,
            angle,
            arm,
            arc,
            height,
            width,
            length: back.distance(front),
        }
    }

    pub fn ring_to_world(&self, rpos: RPos) -> Vec3<Float> {
        let theta = lerp(self.angle -self.arc / 2., self.angle + self.arc / 2., clamp(0.,1., rpos.arc));
        let r = self.arm + (self.height * 2. * rpos.y) / 2.;
        let x = lerp(-self.width / 2., self.width / 2., rpos.x);
        let y = self.origin.x + r * theta.cos();
        let z = self.origin.y + r * theta.sin();
        Vec3::new(x, y, z)
    }

    pub fn width(&self) -> Float {
        self.width.clone()
    }

    pub fn height(&self) -> Float {
        self.height.clone()
    }

    pub fn length(&self) -> Float {
        self.length.clone()
    }

    pub fn set_size(&mut self, width: Float, height: Float) {
        self.width = width;
        self.height = height;
    }

    /// Applies the bounding box filter to a given mesh. 
    pub fn map_mesh(&self, mesh: &mut Mesh) {
        mesh.map_verts(|v| self.ring_to_world(v.into()));
    }
}


#[cfg(test)]
mod tests {
    use crate::export::*;
    use crate::mesh::*;
    use crate::station::ring_space::*;

    #[test]
    fn lerp_test() {
        let result = lerp(0., 1., 0.5);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn ring_space_test() {
        let mut mesh = Mesh::new();
        let rbox = RBox::new(Vec2::zero(), 0., 500., (2. * std::f64::consts::PI) / 8., 32., 32.);
        println!("RBOX = {:?}", rbox);

        mesh.set_name("Curved Box".to_string());
        mesh.add_vertex(Vec3::new(1.,1.,0.));
        mesh.add_vertex(Vec3::new(1.,0.,0.));
        mesh.add_vertex(Vec3::new(1.,1.,1.));
        mesh.add_vertex(Vec3::new(1.,0.,1.));
        mesh.add_vertex(Vec3::new(0.,1.,0.));
        mesh.add_vertex(Vec3::new(0.,0.,0.));
        mesh.add_vertex(Vec3::new(0.,1.,1.));
        mesh.add_vertex(Vec3::new(0.,0.,1.));

        mesh.add_face(vec![0,4,6,2]);
        mesh.add_face(vec![3,2,6,7]);
        mesh.add_face(vec![7,6,4,5]);
        mesh.add_face(vec![5,1,3,7]);
        mesh.add_face(vec![1,0,2,3]);
        mesh.add_face(vec![5,4,0,1]);

        rbox.map_mesh(&mut mesh);
        mesh.normalize(Vec3::new(0., 468., 0.));
        mesh.invert_z();
        
        export_obj(mesh, "ring_space_test".to_string()).unwrap();
    }
}
