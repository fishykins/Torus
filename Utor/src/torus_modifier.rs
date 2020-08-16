use vek::{Vec2, Vec3};
use corale::mesh::*;
use corale::core::*;

/// Represents a single point in ring space. Each value must fall between 0 and 1
#[derive(Clone, Debug)]
pub struct RPos<T> where T: GeoNum {
    x: T,
    y: T,
    arc: T,
}

/// Represents a boundingbox that can translate between ring and world space. 
#[derive(Debug, Clone)]
pub struct TorusModifier<T> where T: GeoNum {
    origin: Vec2<T>,
    angle: f64,
    arm: f64,
    arc: f64,
    height: f64,
    width: f64,
    length: f64,
}

impl<T> RPos<T> where T: GeoNum {
    pub fn new(x: T, y: T, arc: T) -> Self {
        Self {
            arc: maths::clamp(arc, T::zero(), T::one()),
            y: maths::clamp(y, T::zero(), T::one()),
            x: maths::clamp(x, T::zero(), T::one()),
        }
    }
}

impl<T> From<Vertex<T>> for RPos<T> where T: GeoNum {
    fn from(v: Vertex<T>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<T> From<Vec3<T>> for RPos<T> where T: GeoNum {
    fn from(v: Vec3<T>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<T> From<&Vec3<T>> for RPos<T> where T: GeoNum {
    fn from(v: &Vec3<T>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<T> Point<T> for RPos<T> where T: GeoNum  {
    fn from_vec3(pos: Vec3<T>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            arc: pos.z
        }
    }

    fn to_vec3(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.arc)
    }
}

impl<T> TorusModifier<T> where T: GeoNum {
    pub fn new(origin: Vec2<T>, angle: T, arm: T, arc: T, height: T, width: T) -> Self {
        let arc_f64 = arc.to_f64().unwrap();
        let arm_f64 = arm.to_f64().unwrap();
        let theta = (- arc_f64 / 2., arc_f64 / 2.);
        let front = Vec2::new(arm_f64 * theta.1.cos(), arm_f64 * theta.1.sin());
        let back = Vec2::new(arm_f64 * theta.0.cos(), arm_f64 * theta.0.sin());
        Self {
            origin,
            angle: angle.to_f64().unwrap(),
            arm: arm.to_f64().unwrap(),
            arc: arc.to_f64().unwrap(),
            height: height.to_f64().unwrap(),
            width: width.to_f64().unwrap(),
            length: back.distance(front),
        }
    }

    pub fn ring_to_world(&self, vertex: &Vertex<T>) -> Vertex<T> {
        let modifier = RPos::<T>::from(*vertex);
        let theta = maths::lerp(self.angle -self.arc / 2., self.angle + self.arc / 2., maths::clamp(modifier.arc.to_f64().unwrap(), 0.,1.));
        let r = self.arm + (self.height * 2. * modifier.y.to_f64().unwrap()) / 2.;
        let x = T::from_f64(maths::lerp(-self.width / 2., self.width / 2., modifier.x.to_f64().unwrap())).unwrap();
        let y = self.origin.x + T::from_f64(r * theta.cos()).unwrap();
        let z = self.origin.y + T::from_f64(r * theta.sin()).unwrap();
        Vertex::new(x, y, z)
    }

    pub fn width(&self) -> T {
        T::from_f64(self.width).unwrap()
    }

    pub fn height(&self) -> T {
        T::from_f64(self.height).unwrap()
    }

    pub fn length(&self) -> T {
        T::from_f64(self.length).unwrap()
    }

    pub fn set_size(&mut self, width: T, height: T) {
        self.width = width.to_f64().unwrap();
        self.height = height.to_f64().unwrap();
    }
}

impl<T> Filter<T> for TorusModifier<T> where T: GeoNum {
    /// Applies the bounding box filter to a given mesh. 
    fn apply(&self, mesh: &mut Mesh<T>) {
        mesh.map_verts(|v| self.ring_to_world(v));
    }
}

