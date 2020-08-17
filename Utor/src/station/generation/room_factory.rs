use corale::geom::{BoxCollider};
use corale::core::{GridNum};
use crate::station::room::Room;
use super::marching;
use vek::Vec3;

pub enum RoomFactory {
    Marching(usize),
}

pub fn build_room<T>(start_position: Vec3<T>, bounds: &dyn BoxCollider<T>, colliders: &Vec<Room<T>>, factory: RoomFactory) -> Option<Room<T>> where T: GridNum  {
    match factory {
        RoomFactory::Marching(limit) => marching(start_position, bounds, colliders, limit)
    }
}
