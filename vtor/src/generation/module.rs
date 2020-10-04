use vek::{Rgb, Vec2, Extent2};
use prima::geom::{BoundingRect, Line, LineExt};
use prima::render::{RgbImage, Draw};
use rand::prelude::*;

const _HOMAN_WIDTH: f32 = 1.2;
const _HOMAN_HEIGHT: f32 = 2.2;
const MODULE_WIDTH: f32 = 64.;
const _MODULE_HEIGHT: f32 = 32.;
const MODULE_DEPTH: f32 = 128.;

const SPLIT_OFFSET: f32 = 0.2;
const OVERFLOW_ROOMS: usize = 10;

const IMAGE_SCALE: u32 = 4;

#[derive(Clone)]
pub struct Room {
    pub rect: BoundingRect<f32>,
    pub connected: Vec<usize>,
    pub main: bool,
}

#[allow(dead_code)]
pub struct Module {
    pub bounds: BoundingRect<f32>,
    pub rooms: Vec<Room>,
}

impl Room {
    pub fn new(rect: BoundingRect<f32>) -> Self {
        Self {
            rect,
            connected: Vec::new(),
            main: false,
        }
    }

    pub fn size(&self) -> Extent2<f32> {
        self.rect.size()
    }
}

impl Module {
    pub fn new(seed: u64, room_count: usize) -> Self {
        let mut bounds = BoundingRect::new_empty(Vec2::zero());
        bounds.max = Vec2::new(MODULE_WIDTH, MODULE_DEPTH);
        bounds.make_valid();

        let mut rooms = vec![Room::new(bounds.clone())];
        let mut rng = StdRng::seed_from_u64(seed);


        for _ in 0.. (room_count - 1) + OVERFLOW_ROOMS {

            let mut index = 0;

            let v: f32 = rng.gen();

            if v > 0.55 {
                let mut largest_area = 0.;
                for (i, room) in rooms.iter().enumerate() {
                    let area = room.size().w * room.size().h;
                    if area > largest_area {
                        largest_area = area;
                        index = i;
                    }
                }
            } else if v > 0.1 {
                let mut largest_disparity = 0.;
                for (i, room) in rooms.iter().enumerate() {
                    let disp = (room.size().w - room.size().h).abs();
                    if disp > largest_disparity {
                        largest_disparity = disp;
                        index = i;
                    }
                }
            } else {
                index = rng.gen_range::<usize, usize, usize>(0, rooms.len());
            }
            
            let random_room = rooms[index].clone();
            let extent = random_room.size();
            let r: f32 = rng.gen_range::<f32, f32, f32>(SPLIT_OFFSET, 1. - SPLIT_OFFSET);

            rooms.remove(index);
            if extent.w > extent.h {
                let split = random_room.rect.split_at_x(random_room.rect.min.x + r * extent.w);
                let room_a = Room::new(split[0].clone().made_valid());
                let room_b = Room::new(split[1].clone().made_valid());
                rooms.push(room_a);
                rooms.push(room_b);
            } else {
                let split = random_room.rect.split_at_y(random_room.rect.min.y + r * extent.h);
                let room_a = Room::new(split[0].clone().made_valid());
                let room_b = Room::new(split[1].clone().made_valid());
                rooms.push(room_b);
                rooms.push(room_a);
            }
        }

        // Remove some random rooms
        for _ in 0..OVERFLOW_ROOMS {
            let index = rng.gen_range(0, rooms.len());
            rooms.remove(index);
        }

        // Find main room
        let mut largest_area = 0.;
        let mut main_room = 0;

        for (i, room) in rooms.iter().enumerate() {
            let area = room.size().w * room.size().h;
            if area > largest_area {
                largest_area = area;
                main_room = i;
            }
        }

        rooms[main_room].main = true;
        println!("Room count: {}", rooms.len());

        // Connect rooms
        // Find any obvious links, such as intersecting or touching rooms
        let rooms_cloned = rooms.clone();

        for i in 0..rooms.len() {

            let adj = find_adjacant_rooms(&rooms, i);
            println!("Room {}: {:?}", i, adj);

            let a = &mut rooms[i];
            let mut nearest: Option<f32> = None;
            let mut nearest_index = 0;

            for (j, b) in rooms_cloned.iter().enumerate().map(|(i, b)| (i, b.rect)) {
                if i == j {
                    continue;
                }

                let dist = a.rect.center().distance(b.center());
                if nearest.is_none() {
                    nearest = Some(dist);
                } else {
                    if dist < nearest.unwrap() {
                        nearest = Some(dist);
                        nearest_index = j;
                    }
                }
            }

            if nearest.is_some() {
                if !a.connected.contains(&nearest_index) {
                    a.connected.push(nearest_index);
                }
                if !rooms[nearest_index].connected.contains(&i) {
                    rooms[nearest_index].connected.push(i);
                }
            }
        }

        // Find "islands" of connected rooms
        let mut islands = Vec::new();
        recursively_collect_islands(&rooms, &mut islands, 0);
        println!("Found {} islands: ", islands.len());
        
        // Link islands
        

        Self {
            bounds,
            rooms,
        }
    }

    pub fn export(&self) {
        let mut img = RgbImage::new(self.bounds.max.x as u32 * IMAGE_SCALE, self.bounds.max.y as u32 * IMAGE_SCALE);

        for (i, room) in self.rooms.iter().enumerate() {
            let boundingbox = BoundingRect {
                min: room.rect.min * IMAGE_SCALE as f32,
                max: room.rect.max * IMAGE_SCALE as f32,
            }.made_valid();

            let colour = if room.main {
                Rgb::new(0,0,255)
            } else {
                Rgb::new(255,0,0)
            };

            boundingbox.into_rect().draw(&mut img, colour);

            for j in room.connected.iter() {
                if *j > i {
                    // Only draws the line if target is bigger index. Prevents doubles
                    let a = self.rooms[i].rect;
                    let b = self.rooms[*j].rect;

                    let line = Line {
                        start: a.center() * IMAGE_SCALE as f32,
                        end: b.center() * IMAGE_SCALE as f32,
                    };
                    line.draw(&mut img, Rgb::new(0,255,0));
                }
            }
        }

        img.save("../bin/module_export.png").unwrap();
    }
}

fn find_adjacant_rooms(rooms: &Vec<Room>, index: usize) -> Vec<usize> {
    let room = &rooms[index];
    let rect = room.rect;
    let mut neighbors = Vec::new();

    for (i, rhs) in rooms.iter().enumerate().map(|(i, rhs)| (i, rhs.rect)) {
        if i == index {
            continue;
        }
        let x_overlap = (rect.min.x >= rhs.min.x && rect.min.x <= rhs.max.x) || (rect.max.x >= rhs.min.x && rect.max.x <= rhs.max.x);
        let y_overlap = (rect.min.y >= rhs.min.y && rect.min.y <= rhs.max.y) || (rect.max.y >= rhs.min.y && rect.max.y <= rhs.max.y);
        if x_overlap && y_overlap {
            neighbors.push(i);
        }
    }

    neighbors
}

fn recursively_collect_islands(rooms: &Vec<Room>, islands: &mut Vec<Vec<usize>>, room_index: usize) {
    let room = &rooms[room_index];
    let mut island_index: Option<usize> = None;

    // Check to see if we have an island
    for (i, island) in islands.iter().enumerate() {
        if island.contains(&room_index) {
            // All good, carry on
            island_index = Some(i);
            break;
        }
    }

    if island_index.is_none() {
        // We need to initiate an island
        island_index = Some(islands.len());
        islands.push(vec!(room_index));
    }

    let i = island_index.unwrap(); // Will allwways work as we ensure it is set
    // We are already in the island- add our connected too

    let mut room_q = Vec::new();

    for c in room.connected.iter() {
        if !islands[i].contains(c) {
            // This room has not been seen before- recursively add it!
            islands[i].push(*c);
            room_q.push(*c);
        }
    }

    // Once we have added all the unseen connected rooms to our islands, run through them recursively
    for r in room_q.iter() {
        recursively_collect_islands(&rooms, islands, *r);
    }

    // Once we reach this point, we need to find a new island entierly. 
    for r in room_index..rooms.len() {
        let mut has_island = false;
        for (i, island) in islands.iter().enumerate() {
            if island.contains(&r) {
                has_island = true;
                break;
            }
        }

        if !has_island {
            recursively_collect_islands(&rooms, islands, r);
        }
    }
}

#[test]
fn module_test() {
    let module = Module::new(0785, 7);
    module.export();
}