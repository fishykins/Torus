use super::{Room, LinkType};
use vek::{Rgb, Vec2};
use prima::geom::{BoundingRect, Line, LineExt};
use prima::render::{RgbImage, Draw};
use prima::core::maths::*;
use rand::prelude::*;


const _HOMAN_WIDTH: f32 = 1.2;
const _HOMAN_HEIGHT: f32 = 2.2;
const MODULE_WIDTH: f32 = 64.;
const _MODULE_HEIGHT: f32 = 32.;
const MODULE_DEPTH: f32 = 128.;

const SPLIT_OFFSET: f32 = 0.2;
const OVERFLOW_ROOMS: usize = 10;

const IMAGE_SCALE: u32 = 4;

#[allow(dead_code)]
pub struct Module {
    pub bounds: BoundingRect<f32>,
    pub rooms: Vec<Room>,
    most_junctions: usize,
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

        println!("Room count: {}", rooms.len());

        // Connect rooms
        for i in 0..rooms.len() {
            for j in find_adjacant_rooms(&rooms, i) {
                if i == j {
                    continue;
                }
                connect_rooms(&mut rooms, i, j, LinkType::Direct);
            }
            if rooms[i].connected().len() == 0 {
                //Nearest room instead
                let j = find_nearest_room(&rooms, i);
                connect_rooms(&mut rooms, i, j, LinkType::Tunnel);
            }

            println!("Room {} is connected to {:?}", i, rooms[i].connected());
        }

        // Find "islands" of connected rooms
        let mut islands = Vec::new();
        recursively_collect_islands(&rooms, &mut islands, 0, 0);
        println!("Found {} islands: ", islands.len());

        for i in islands.iter() {
            println!("  {:?}", i);
        }
        
        // Link islands
        for island in islands.iter() {
            let mut nearest = f32::MAX;
            let mut nearest_index = (0, 0);

            for island_room_index in island.iter() {
                for (i, room) in rooms.iter().enumerate() {
                    if !island.contains(&i) {
                        let dist = rooms[*island_room_index].rect.center().distance(room.rect.center());
                        if dist < nearest {
                            nearest = dist;
                            nearest_index = (*island_room_index, i);
                        }
                    }
                }
            }

            if nearest != f32::MAX {
                connect_rooms(&mut rooms, nearest_index.0, nearest_index.1, LinkType::Bridge);
            }
        }

        // At this point, we can navigate from the main room to any other room we like. Time to add some extra pathways

        // Lets try and link every room to the main room directly, using ray casting
        let main_room_pos = rooms[main_room].rect.center();
        for i in 0..rooms.len() {
            if i == main_room {
                continue;
            }

            let ray = Line {
                start: rooms[i].rect.center(),
                end: main_room_pos,
            };

            // Check to see if this ray intersects any rooms other than ours or main
            let mut intersect = false;
            for (j, r2) in rooms.iter().enumerate() {
                if j == main_room || j == i {
                    continue;
                }

                if ray.intersects_rect(&r2.rect.into_rect()) {
                    intersect = true;
                    break;
                }
            }

            if !intersect && !rooms[i].is_linked(main_room) {
                connect_rooms(&mut rooms, i, main_room, LinkType::Main);
            }
        }

        // Statistical analysis
        let mut most_junctions = 1;

        for r in rooms.iter() {
            let len = r.connected().len();
            if  len > most_junctions {
                most_junctions = len;
            }
        }

        for i in 0..rooms.len() {
            let junctions = rooms[i].connected().len() as f32;
            let focus = inverse_lerp(1., most_junctions as f32, junctions);
            rooms[i].value = lerpc(0.,1., focus);
            println!("Room {} has a value of {}", i, rooms[i].value);
        }

        Self {
            bounds,
            rooms,
            most_junctions,
        }
    }

    pub fn export(&self) {
        let mut img = RgbImage::new(self.bounds.max.x as u32 * IMAGE_SCALE, self.bounds.max.y as u32 * IMAGE_SCALE);

        for (i, room) in self.rooms.iter().enumerate() {
            let boundingbox = BoundingRect {
                min: room.rect.min * IMAGE_SCALE as f32,
                max: room.rect.max * IMAGE_SCALE as f32,
            }.made_valid();

            let red: u8 = lerpc(0., 255., 1. - room.value) as u8;
            let green: u8 = lerpc(0., 255., room.value) as u8;

            boundingbox.into_rect().draw(&mut img, Rgb::new(red,green,0));

            for j in room.links() {
                if j.target > i {
                    // Only draws the line if target is bigger index. Prevents doubles
                    let a = self.rooms[i].rect;
                    let b = self.rooms[j.target].rect;

                    let line = Line {
                        start: a.center() * IMAGE_SCALE as f32,
                        end: b.center() * IMAGE_SCALE as f32,
                    };

                    let colour = match j.link_type {
                        LinkType::Direct => Rgb::new(0,255,0),
                        LinkType::Bridge => Rgb::new(0,255,255),
                        LinkType::Tunnel => Rgb::new(255,255,0),
                        LinkType::Main => Rgb::new(82,56,255),
                    };

                    line.draw(&mut img, colour);
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

fn find_nearest_room(rooms: &Vec<Room>, index: usize) -> usize {
    let rect = &rooms[index].rect;
    let mut nearest = index;
    let mut nearest_distance = f32::MAX;

    for (i, rhs) in rooms.iter().enumerate().map(|(i, rhs)| (i, rhs.rect)) {
        if i == index {
            continue;
        }
        let dist = rect.center().distance(rhs.center());
        if dist < nearest_distance {
            nearest = i;
            nearest_distance = dist;
        }
    }
    nearest
}

fn connect_rooms(rooms: &mut Vec<Room>, a: usize, b: usize, link_type: LinkType) {
    rooms[a].link(b, link_type);
    rooms[b].link(a, link_type);
}

fn recursively_collect_islands(rooms: &Vec<Room>, islands: &mut Vec<Vec<usize>>, room_index: usize, depth: usize) {
    let room = &rooms[room_index];
    let mut island_index: Option<usize> = None;

    println!("Running Recursive collect on room {} (depth {})...", room_index, depth);

    // Check to see if we have an island
    for (i, island) in islands.iter().enumerate() {
        if island.contains(&room_index) {
            // All good, carry on
            island_index = Some(i);
            println!("    found island index {}...", i);
            break;
        }
    }

    if island_index.is_none() {
        // We need to initiate an island
        println!("    no island found- made island index {}...", islands.len());
        island_index = Some(islands.len());
        islands.push(vec!(room_index));
    }

    let i = island_index.unwrap(); // Will allwways work as we ensure it is set
    
    // We are already in the island- add our connected too
    let mut room_q = Vec::new();

    for c in room.connected().iter() {
        println!("    found connected room {}", c);
        if !islands[i].contains(c) {
            // This room has not been seen before- recursively add it!
            islands[i].push(*c);
            room_q.push(*c);
        }
    }

    // Once we have added all the unseen connected rooms to our islands, run through them recursively
    for r in room_q.iter() {
        recursively_collect_islands(&rooms, islands, *r, depth + 1);
    }

    // Once we reach this point, we need to find a new island entierly
    if depth != 0 {
        return;
    }

    for r in room_index + 1 .. rooms.len() {
        let mut has_island = false;
        for island in islands.iter() {
            if island.contains(&r) {
                has_island = true;
                break;
            }
        }

        if !has_island {
            recursively_collect_islands(&rooms, islands, r, depth + 1);
        }
    }
}

#[test]
fn module_test() {
    let module = Module::new(78941, 17);
    module.export();
}