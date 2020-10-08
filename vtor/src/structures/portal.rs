use prima::core::maths::*;
use vek::{Rect, Vec2};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum NESW {
    North,
    East,
    South,
    West
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
}

/// A portal is just a door attached to a room, refferenced in links
#[derive(Clone, Copy, PartialEq)]
pub struct Portal {
    pub position: Vec2<f32>,
    orientation: Orientation,
}

impl Portal {
    pub fn new(position: Vec2<f32>, direction: NESW) -> Self {
        let orientation = if direction == NESW::North || direction == NESW::South {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        };

        Self {
            position,
            orientation,
        }
    }

    pub fn from_rect_edge(rect: &Rect<f32,f32>, direction: NESW, offset: f32) -> Self {
        Self::new(match direction {
            NESW::North => Vec2::new(lerp(rect.x, rect.x + rect.w, offset), rect.y + rect.h),
            NESW::South => Vec2::new(lerp(rect.x, rect.x + rect.w, offset), rect.y),
            NESW::East => Vec2::new(rect.x + rect.w, lerp(rect.y, rect.y + rect.h, offset)),
            NESW::West => Vec2::new(rect.x, lerp(rect.y, rect.y + rect.h, offset)),
        }, direction)
    }

    /// Finds the position on the given rectangle edge that this sits
    pub fn position_on_edge(&self, rect: &Rect<f32,f32>) -> Option<(NESW, f32)> {

        let x = self.position.x;
        let y = self.position.y;

        let x_in_bounds = x >= rect.x && x <= rect.x + rect.w;
        let y_in_bounds = y >= rect.y && y <= rect.y + rect.h;

        if self.orientation == Orientation::Vertical {
            if y_in_bounds && x == rect.x {
                // West edge
                return Some((NESW::West, inverse_lerp(rect.y, rect.y + rect.h, y)));
            } else if y_in_bounds && x == rect.x + rect.w {
                // East edge
                return Some((NESW::East, inverse_lerp(rect.y, rect.y + rect.h, y)));
            }
            return None;
        };
        
        if x_in_bounds && y == rect.y {
            // South edge
            return Some((NESW::South, inverse_lerp(rect.x, rect.x + rect.w, x)));
        } else if x_in_bounds && y == rect.y + rect.h {
            // North edge
            return Some((NESW::North, inverse_lerp(rect.x, rect.x + rect.w, x)));
        }
        None
    }
}