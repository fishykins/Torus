use vek::{Vec2, Vec3};
use crate::mesh::*;
use crate::maths;
use crate::mesh_modifier::MeshModifier;

type Float = f64;

/// Represents a single point in ring space. Each value must fall between 0 and 1
#[derive(Clone, Debug)]
pub struct RPos {
    arc: Float,
    y: Float,
    x: Float,
}

/// Represents a boundingbox that can translate between ring and world space. 
#[derive(Clone, Debug)]
pub struct TorusModifier {
    origin: Vec2<Float>,
    angle: Float,
    arm: Float,
    arc: Float,
    height: Float,
    width: Float,
    length: Float,
}

impl RPos {
    pub fn new(x: f64, y: f64, arc: f64) -> Self {
        Self {
            arc: maths::clamp(0., 1., arc).into(),
            y: maths::clamp(0., 1., y),
            x: maths::clamp(0., 1., x),
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

impl TorusModifier {
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

    pub fn ring_to_world(&self, modifier: RPos) -> Vec3<Float> {
        let theta = maths::lerp(self.angle -self.arc / 2., self.angle + self.arc / 2., maths::clamp(0.,1., modifier.arc));
        let r = self.arm + (self.height * 2. * modifier.y) / 2.;
        let x = maths::lerp(-self.width / 2., self.width / 2., modifier.x);
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
}

impl MeshModifier for TorusModifier {
    /// Applies the bounding box filter to a given mesh. 
    fn apply(&self, mesh: &Mesh) -> Mesh {
        let mut new_mesh = mesh.clone();
        new_mesh.map_verts(|v| self.ring_to_world(v.into()));
        new_mesh
    }
}

