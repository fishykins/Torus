use vek::{Vec2, Vec3};

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

/// Represents a single point in ring space. Each value must fall between 0 and 1
pub struct RPos {
    arc: f32,
    y: f32,
    x: f32,
}

/// Represents a boundingbox that can translate between ring and world space. 
pub struct RBox {
    origin: Vec2<f32>,
    arm: f32,
    arc: f32,
    height: f32,
    width: f32,
}

impl RPos {
    pub fn new(arc: f32, y: f32, x: f32) -> Self {
        Self {
            arc: clamp(0., 1., arc),
            y: clamp(0., 1., y),
            x: clamp(0., 1., x),
        }
    }
}

impl RBox {
    pub fn new(origin: Vec2<f32>, arm: f32, arc: f32, height: f32, width: f32) -> Self {
        Self {
            origin,
            arm,
            arc,
            height,
            width,
        }
    }

    pub fn ring_to_world(&self, rpos: RPos) -> Vec3<f32> {
        let theta = lerp(-self.arc / 2., self.arc / 2., clamp(0.,1., rpos.arc));
        let r = self.arm + self.height / 2. - rpos.y;
        let x = lerp(-self.width / 2., self.width / 2., rpos.x);
        Vec3::new(x, self.origin.x + r * theta.cos(), self.origin.y + r * theta.sin())
    }
}