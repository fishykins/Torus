use crate::station::config::*;
use crate::station::room::Room;
use corale::geom::{Cube, BoxCollider, BoundingBox};
use corale::core::*;
use rand::prelude::*;
use vek::Vec3;

/// Contains a map of all interior structures of a module
pub struct ModuleMap<T, R> where T: OrdNum, R: Cube<T> + BoxCollider<T> {
    spaces: Vec<Box<R>>,
    _var: T,
}

pub fn build<T>(bounds: &BoundingBox<T>, cfg: &Config) where T: OrdNum {
    let mut rng = thread_rng();
    let mut spaces = Vec::new();
    let width = bounds.width().to_f64().unwrap();
    let height = bounds.height().to_f64().unwrap();
    let length = bounds.depth().to_f64().unwrap();

    // generate some big rooms
    let room_width =  rng.gen_range(width / 2., width);
    let room_length =  rng.gen_range(length / 16., length);
    let room_height =  cfg.humans.dimensions().y;

    //calculate max point this room can fit in
    let pos_max = Vec3::new(width - room_width, height - room_height, length - room_length);
    let pos_actual = Vec3::new(
        rng.gen_range(0., pos_max.x),
        rng.gen_range(0., pos_max.y),
        rng.gen_range(0., pos_max.z),
    );

    let room_bounds = BoundingBox::new(pos_actual, pos_actual + Vec3::new(room_width, room_height, room_length));
    let room = Room::new(room_bounds);
    spaces.push(Box::new(room));
}