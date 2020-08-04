use vek::{Vec2, Vec3, Vec4};
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
        return b + ( range / amount);
    } else {
        let range = b - a;
        return a + ( range / amount);
    }
}


/// Represents a segment of a cylinder's circumference as a "flat" plane.
/// This is useful for creating uniform surfaces, such as floors and ceilings.
pub struct CPlane {
    origin: Vertex,
    arc: f32,
    radius: f32,
    width: f32,
    b_l: Vertex,
    b_r: Vertex,
    f_l: Vertex,
    f_r: Vertex,
}

impl CPlane {
    pub fn new(origin: Vertex, arc: f32, radius: f32, width: f32) -> Self {
        let arc_range = (- arc / 2., arc / 2.);
        let arc_cos = (arc_range.0.cos(), arc_range.1.cos());
        let arc_sin = (arc_range.0.sin(), arc_range.1.sin());
        let w = width / 2.;

        Self {
            origin,
            arc,
            radius,
            width,
            f_l: origin + Vertex::new(-w, radius * arc_cos.1, radius * arc_sin.1),
            f_r: origin + Vertex::new(w, radius * arc_cos.1, radius * arc_sin.1),
            b_l: origin + Vertex::new(-w, radius * arc_cos.0, radius * arc_sin.0),
            b_r: origin + Vertex::new(w, radius * arc_cos.0, radius * arc_sin.0),
        }
    }

    /// Back-left point
    pub fn b_l(&self) -> Vertex {
        self.b_l.clone()
    }

    /// Back-right point
    pub fn b_r(&self) -> Vertex {
        self.b_r.clone()
    }

    /// Front-left point
    pub fn f_l(&self) -> Vertex {
        self.f_l.clone()
    }

    /// Front-right point
    pub fn f_r(&self) -> Vertex {
        self.f_r.clone()
    }

    /// Gets a pair of points along the plane, lerping from back to front along the arc.
    /// value must be between 0 and 1, and will be clamped if it falls outside this boundary
    pub fn slice_edge(&self, value: f32) -> (Vertex, Vertex) {
        let theta = lerp(-self.arc / 2., self.arc / 2., clamp(0.,1., value));
        let w = self.width / 2.;
        let left = self.origin + Vertex::new(-w, self.radius * theta.cos(), self.radius * theta.sin());
        let right = self.origin + Vertex::new(w, self.radius * theta.cos(), self.radius * theta.sin());
        (left, right)
    }
}