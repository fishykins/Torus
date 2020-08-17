
use corale::geom::{BoxCollider};
use corale::core::{Direction, GridNum};
use crate::station::room::Room;
use crate::growable_box::*;
use vek::Vec3;
use rand::prelude::*;

pub fn marching<T>(start_position: Vec3<T>, bounds: &dyn BoxCollider<T>, colliders: &Vec<Room<T>>, limit: usize) -> Option<Room<T>> where T: GridNum {
    // Create the room
    let mut room_box = GrowableBox::new(start_position);
    let mut rng = thread_rng();
    let mut expanding = true;

    let mut direction = Direction::Right;
    let mut expansions = rng.gen_range(0, limit);

    while expanding {

        let mut change_direction = false;

        if expansions > 0 {
            //Expand in the current direction, if possible
            expansions -= 1;
            let mut room_clone = room_box.clone();
            room_clone.expand(&direction, T::one());

            if bounds.contains(&room_clone) {
                //Check we dont intersect another room
                for r in colliders {
                    if room_clone.intersects(r) {
                        // the new expanded room intersects something- change direction.
                        change_direction = true;
                        break;
                    }
                }
                //If we made it here, the room is able to expand- update to the expanded room and loop
                room_box = room_clone;
            } else {
                // This falls outside bounds, change direction.
                change_direction = true;
            }
        } else {
            //Ran out of expansions, change direction.
            change_direction = true;
        }

        if change_direction {
            expansions = rng.gen_range(0, limit);
            direction = direction.next();
            if direction.is_none() {
                expanding = false;
            }
        }
    }

    //Check the final product is indeed legitimate
    if bounds.contains(&room_box) {
        //Check we dont intersect another room
        let mut intersects_another_room = false;
        for r in colliders {
            if room_box.intersects(r) {
                // the new expanded room intersects something- change direction.
                intersects_another_room = true;
                break;
            }
        }

        if !intersects_another_room {
            return Some(Room::new(room_box));
        }
    }

    return None;
}