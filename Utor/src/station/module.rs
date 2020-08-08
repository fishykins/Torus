use vek::{Vec2};
use crate::station::{ring_space::*};
use crate::mesh::*;
use crate::primatives::*;
use crate::{box_collider::BoxCollider, bounding_box::BoundingBox, growable_box::{GrowableBox, Direction}};
use rand::prelude::*;

const LENGTH_K: Float = 3.;
const WIDTH_K: Float = 1.;
const HEIGHT_K: Float = 1.;

const AREA_INNER: (f64,f64,f64) = (0.5, 0.5, 1.);
const GRID_SIZE: (usize,usize,usize) = (5,5,14);

const ROOM_COUNT: usize = 5;
const ROOM_SIZE_LIMMIT: usize = 3;

type Float = f64;

#[derive(Clone)]
pub struct Module {
    index: usize,
    bounding_box: RBox,
    mesh: Mesh,
    rooms: Vec<GrowableBox>,
}


impl Module {
    /// Creates a new module. 
    /// theta: the angle size of the segment's arc
    /// radius: the radius from the ring center to the middle of the module
    /// mesh: parent mesh to clone from. 
    pub fn new(index: usize, arc: Float, radius: Float, mesh: &Mesh) -> Self {
        let angle = index as Float * arc;
        let mut bounding_box = RBox::new(Vec2::zero(), angle, radius, arc, 0., 0.);
        let k = bounding_box.length() / LENGTH_K;
        bounding_box.set_size(k * WIDTH_K, k * HEIGHT_K);
        let mut new_mesh = mesh.clone();
        let name = format!("module_{}", index);
        new_mesh.set_name(name);
        //let rooms = UGrid3<Room> = UGrid3::new(Vec3::new(GRID_SIZE.0, GRID_SIZE.1, GRID_SIZE.2));

        let mut me = Self {
            index,
            rooms: Vec::new(),
            bounding_box,
            mesh: new_mesh,
        };

        me.generate_random_rooms();
        me
    }

    pub fn generate_all_rooms(&mut self) {
        let bounds = Vec3::new(GRID_SIZE.0 as i8, GRID_SIZE.1 as i8, GRID_SIZE.2 as i8);

        for x in 0..GRID_SIZE.0 {
            for y in 0..GRID_SIZE.1 {
                for z in 0..GRID_SIZE.2 {
                    let pos: Vec3<i8> = Vec3::new(x as i8, y as i8,z  as i8);
                    let new_room = GrowableBox::new(pos);
                    self.rooms.push(new_room);
                }
            }
        }
    }

    pub fn generate_random_rooms(&mut self) {
        let bounds = BoundingBox::new(Vec3::zero(), Vec3::new(GRID_SIZE.0 as i8, GRID_SIZE.1 as i8, GRID_SIZE.2 as i8));
        let mut rooms_left = ROOM_COUNT;
        let mut atempts = 0;
        let mut rng = thread_rng();

        while rooms_left > 0 && atempts < 32 {
            // Individual room starts here
            atempts += 1;
            // Get random position
            let x = rng.gen_range(0, GRID_SIZE.0);
            let y = rng.gen_range(0, GRID_SIZE.1);
            let z = rng.gen_range(0, GRID_SIZE.2);
            let pos: Vec3<i8> = Vec3::new(x as i8, y as i8,z  as i8);
            // Create the room
            let mut room = GrowableBox::new(pos);
            let mut expanding = true;

            println!("Starting Room {} at {}...", ROOM_COUNT - rooms_left, pos);

            let mut direction = Direction::Right;
            let mut expansions = rng.gen_range(0, ROOM_SIZE_LIMMIT);

            while expanding {

                let mut change_direction = false;

                if expansions > 0 {
                    //Expand in the current direction, if possible
                    expansions -= 1;
                    let mut room_clone = room.clone();
                    room_clone.expand(&direction, 1);

                    if bounds.contains(&room_clone) {
                        //Check we dont intersect another room
                        for r in &self.rooms {
                            if room_clone.intersects(r) {
                                // the new expanded room intersects something- change direction.
                                change_direction = true;
                                break;
                            }
                        }
                        //If we made it here, the room is able to expand- update to the expanded room and loop
                        room = room_clone;
                    } else {
                        // This falls outside bounds, change direction.
                        change_direction = true;
                    }
                } else {
                    //Ran out of expansions, change direction.
                    change_direction = true;
                }

                if change_direction {
                    expansions = rng.gen_range(0, ROOM_SIZE_LIMMIT);
                    match direction.clone() {
                        Direction::Left => direction = Direction::Right,
                        Direction::Right => direction = Direction::Up,
                        Direction::Up => direction = Direction::Down,
                        Direction::Down => direction = Direction::Front,
                        Direction::Front => direction = Direction::Back,
                        Direction::Back | Direction::None => {
                            //There are no more directions to look in, stop
                            expanding = false;
                        },
                    }
                }
            }

            //Check the final product is indeed legitimate
            if bounds.contains(&room) {
                //Check we dont intersect another room
                let mut intersects_another_room = false;
                for r in &self.rooms {
                    if room.intersects(r) {
                        // the new expanded room intersects something- change direction.
                        intersects_another_room = true;
                        break;
                    }
                }

                if !intersects_another_room {
                    self.rooms.push(room);
                    rooms_left -= 1;
                }
            }
        }
    }

    pub fn build_rooms(&self, mesh: &mut Mesh) {
        let step = Vec3::new(
            AREA_INNER.0 / GRID_SIZE.0 as f64, 
            AREA_INNER.1 / GRID_SIZE.1 as f64, 
            AREA_INNER.2 / GRID_SIZE.2 as f64
        );

        let offset = Vec3::new(
            (1. - AREA_INNER.0) /2. + step.x / 2., 
            (1. - AREA_INNER.1) /2. + step.y / 2., 
            (1. - AREA_INNER.2) /2. + step.z / 2.
        );

        for room in &self.rooms {
            let pos_a = Vec3::new(
                offset.x + (step.x * 1.) * room.min().x as f64 - (step.x * 0.5), 
                offset.y + (step.y * 1.) * room.min().y as f64 - (step.y * 0.5), 
                offset.z + (step.z * 1.) * room.min().z as f64 - (step.z * 0.5)
            );
            let pos_b = Vec3::new(
                offset.x + (step.x * 1.) * room.max().x as f64 + (step.x * 0.5), 
                offset.y + (step.y * 1.) * room.max().y as f64 + (step.y * 0.5), 
                offset.z + (step.z * 1.) * room.max().z as f64 + (step.z * 0.5)
            );
            println!("Building Room {} -> {}...", room.min(), room.max());
            make_box(pos_a, pos_b, mesh);
        }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    pub fn build(&self) -> Mesh {
        let mut mesh = self.mesh.clone();
        self.build_rooms(&mut mesh);
        self.bounding_box.map_mesh(&mut mesh);
        mesh.invert_z();
        mesh
        //export_obj(mesh, &name, &name).unwrap();
    }

    pub fn index(&self) -> usize {
        self.index
    }

}