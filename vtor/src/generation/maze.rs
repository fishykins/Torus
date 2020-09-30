use corale::geom::{BoundingBox, Cube};
use corale::core::{GeoNum};
use vek::Vec3;
use rand::prelude::*;

enum Axis {
    X,
    Y,
    Z,
}

pub fn generate<T>() where T: GeoNum {
    let mut tree = vec![BoundingBox::new(Vec3::zero(), Vec3::new(32., 32., 128.))];

    //first, split the box into a kdtree
    let mut axis = Axis::X;
    let mut divides = 4;
    let mut rng = thread_rng();

    while divides > 0 {
        let i = rng.gen_range(0, tree.len() -1);
        let bbox = &tree[i];

        match axis {
            Axis::X => {
                let split = rng.gen_range(bbox.min().x, bbox.max().x);
                let left = BoundingBox::new(bbox.min(), Vec3::new(split, bbox.max().y, bbox.max().z));
                let right = BoundingBox::new(Vec3::new(split, bbox.min().y, bbox.min().z), bbox.max());
                tree[i] = left;
                tree.push(right);
                axis = Axis::Z;
            },
            Axis::Y => {

            },
            Axis::Z => {
                let split = rng.gen_range(bbox.min().z, bbox.max().z);
                let left = BoundingBox::new(bbox.min(), Vec3::new(bbox.max().x, bbox.max().y, split));
                let right = BoundingBox::new(Vec3::new(bbox.min().x, bbox.min().y, split), bbox.max());
                tree[i] = left;
                tree.push(right);
                axis = Axis::X;
            }
        }
        divides -= 1;
    }

    // now we have our regions, create a room in each one. This can be in tetris or maze format
}