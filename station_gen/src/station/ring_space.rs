use vek::{Vec2, Vec3};
use crate::mesh::*;

fn clamp(min: f32, max: f32, value: f32) -> f32 {
    if value > max {
        return max;
    } else if value < min {
        return min;
    }
    value
}

/// Lerp between a and b by amount (0 - 1)
fn lerp(a: f32, b: f32, amount: f32) -> f32 {
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
    arc: f32,
    y: f32,
    x: f32,
}

/// Represents a boundingbox that can translate between ring and world space. 
#[derive(Clone, Debug)]
pub struct RBox {
    origin: Vec2<f32>,
    arm: f32,
    arc: f32,
    height: f32,
    width: f32,
    length: f32,
}

impl RPos {
    pub fn new(x: f32, y: f32, arc: f32) -> Self {
        Self {
            arc: clamp(0., 1., arc),
            y: clamp(0., 1., y),
            x: clamp(0., 1., x),
        }
    }
}

impl From<Vec3<f32>> for RPos {
    fn from(v: Vec3<f32>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<&Vec3<f32>> for RPos {
    fn from(v: &Vec3<f32>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl RBox {
    pub fn new(origin: Vec2<f32>, arm: f32, arc: f32, height: f32, width: f32) -> Self {
        let theta = (- arc / 2., arc / 2.);
        let front = Vec2::new(arm * theta.1.cos(), arm * theta.1.sin());
        let back = Vec2::new(arm * theta.0.cos(), arm * theta.0.sin());
        Self {
            origin,
            arm,
            arc,
            height,
            width,
            length: back.distance(front),
        }
    }

    pub fn ring_to_world(&self, rpos: RPos) -> Vec3<f32> {
        println!("RPOS: {:?}", rpos);
        let theta = lerp(-self.arc / 2., self.arc / 2., clamp(0.,1., rpos.arc));
        println!("    theta = {}", theta);
        let r = self.arm + (self.height * 2. * rpos.y) / 2.;
        println!("    r = {}", r);
        let x = lerp(-self.width / 2., self.width / 2., rpos.x);
        println!("    x = {}", x);
        let y = self.origin.x + r * theta.cos();
        println!("    y = {}", y);
        let z = self.origin.y + r * theta.sin();
        println!("    z = {}", z);
        Vec3::new(x, y, z)
    }

    pub fn width(&self) -> f32 {
        self.width.clone()
    }

    pub fn height(&self) -> f32 {
        self.height.clone()
    }

    pub fn length(&self) -> f32 {
        self.length.clone()
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
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
    use vek::*;

    #[test]
    fn lerp_test() {
        let result = lerp(0., 1., 0.5);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn ring_space_test() {
        let mut mesh = Mesh::new();
        let rbox = RBox::new(Vec2::zero(), 500., (2. * std::f64::consts::PI as f32) / 8., 32., 32.);
        println!("RBOX = {:?}", rbox);

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
        
        export_obj(mesh, "curved_box", "ring_space_test").unwrap();
    }
}
