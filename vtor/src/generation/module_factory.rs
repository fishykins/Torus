use crate::config::ModuleCfg;
use super::intersect::{Intersect, IntersectRef};
use super::{Room, LinkType, IMG_SCALE};
use crate::geom::Compass;

use vek::{Rgb, Vec2};
use prima::geom::{BoundingRect, Line, LineExt};
use prima::render::{RgbImage, Draw};
use prima::core::maths::*;
use rand::prelude::*;

#[allow(dead_code)]
pub struct ModuleFactory {
    pub bounds: BoundingRect<f32>,
    pub rooms: Vec<Room>,
    pub intersects: Vec<Intersect>,
    pub islands: Vec<Vec<usize>>,
    most_junctions: usize,
}

impl ModuleFactory {
    /// Generates a brand new module using the given config. Will do the absolute minimum generation and leave pathfinding execution order to the end user.
    pub fn new(config: ModuleCfg) -> Self {
        let mut bounds = BoundingRect::new_empty(Vec2::zero());
        bounds.max = Vec2::new(config.extent().w, config.extent().h);
        bounds.make_valid();

        let mut rooms = vec![Room::new(bounds.clone())];
        let mut rng = StdRng::seed_from_u64(config.seed);

        let mut offset = clamp01(config.split_offset);
        let degredation = clamp01(config.split_degredation);

        for _ in 0..config.divisions {

            let mut index = 0;

            let v: f32 = rng.gen();

            if v > config.divide_area_chance {
                let mut largest_area = 0.;
                for (i, room) in rooms.iter().enumerate() {
                    let area = room.size().w * room.size().h;
                    if area > largest_area {
                        largest_area = area;
                        index = i;
                    }
                }
            } else if v > config.divide_disparity_chance {
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
            let r: f32 = rng.gen_range::<f32, f32, f32>(offset, 1. - offset);
            // Degrade offset so it becomes less centered
            offset *= degredation;

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
        for _ in 0.. config.divisions - config.room_count {
            let index = rng.gen_range(0, rooms.len());
            rooms.remove(index);
        }

        println!("Room count: {}", rooms.len());

        Self {
            bounds,
            intersects: Vec::new(),
            rooms,
            islands: Vec::new(),
            most_junctions: 0,
        }
    }

    /// Generates a module with the given config and executes default pathfinding workflow.
    pub fn default(config: ModuleCfg) -> Self {
        let mut module = Self::new(config);
        module.link_rooms(true);
        module.generate_islands();
        module.link_islands();
        module.calculate_statistics();
        return module
    }

    /// As far as possible, connects all rooms to their neighbors. 
    /// If allow_nearest, isolated rooms are allowed to link to their nearest neighbor. 
    pub fn link_rooms(&mut self, allow_nearest: bool) {
        for i in 0..self.rooms.len() {
            for j in find_adjacant_rooms(&self.rooms, i) {
                if i == j {
                    continue;
                }

                if j < i {
                    // intersect has probably allready been found- grab it off the stack
                    
                    
                } else {
                    // Find the intersection line
                    let room = self.rooms[i];
                    let other = self.rooms[j];
                    let west = room.rect.min.x == other.rect.max.x;
                    let east = room.rect.max.x == other.rect.min.x;
                    let north = room.rect.max.y == other.rect.min.y;
                    let south = room.rect.min.y == other.rect.max.y;

                    let line = if north || south {
                        let x_max = room.rect.min.x.max(other.rect.min.x);
                        let x_min = room.rect.max.x.min(other.rect.max.x);
                        let y = if north {
                            room.rect.max.y
                        } else {
                            room.rect.min.y
                        };

                        Line {
                            start: Vec2::new(x_min, y),
                            end: Vec2::new(x_max, y),
                        }
                    } else {
                        let y_max = room.rect.min.y.max(other.rect.min.y);
                        let y_min = room.rect.max.y.min(other.rect.max.y);
                        let x = if east {
                            room.rect.max.x
                        } else if west {
                            room.rect.min.x
                        } else {
                            panic!("cannot generate portal: unable to identify x");
                        };

                        Line {
                            start: Vec2::new(x, y_min),
                            end: Vec2::new(x, y_max),
                        }
                    };

                    let edge = if west {
                        Compass::West
                    } else if east {
                        Compass::East
                    } else if north {
                        Compass::North
                    } else {
                        Compass::South
                    };
                }
                //connect_rooms(&mut self.rooms, i, j, LinkType::Direct);
            }
            if self.rooms[i].connected().len() == 0 && allow_nearest {
                //Nearest room instead
                let j = find_nearest_room(&self.rooms, i);
                connect_rooms(&mut self.rooms, i, j, LinkType::Tunnel);
            }
        }
    }

    pub fn generate_islands(&mut self) {
        let mut islands = Vec::new();
        recursively_collect_islands(&self.rooms, &mut islands, 0, 0);
        self.islands = islands;
    }

    pub fn link_islands(&mut self) {
        for island in self.islands.iter() {
            let mut nearest = f32::MAX;
            let mut nearest_index = (0, 0);

            for island_room_index in island.iter() {
                for (i, room) in self.rooms.iter().enumerate() {
                    if !island.contains(&i) {
                        let dist = self.rooms[*island_room_index].rect.center().distance(room.rect.center());
                        if dist < nearest {
                            nearest = dist;
                            nearest_index = (*island_room_index, i);
                        }
                    }
                }
            }
            if nearest != f32::MAX {
                connect_rooms(&mut self.rooms, nearest_index.0, nearest_index.1, LinkType::Bridge);
            }
        }
    }

    pub fn link_raycasting(&mut self) {
        // Find main room
        let mut largest_area = 0.;
        let mut main_room = 0;

        for (i, room) in self.rooms.iter().enumerate() {
            let area = room.size().w * room.size().h;
            if area > largest_area {
                largest_area = area;
                main_room = i;
            }
        }

        // Lets try and link every room to the main room directly, using ray casting
        let main_room_pos = self.rooms[main_room].rect.center();
        for i in 0..self.rooms.len() {
            if i == main_room {
                continue;
            }

            let ray = Line {
                start: self.rooms[i].rect.center(),
                end: main_room_pos,
            };

            // Check to see if this ray intersects any rooms other than ours or main
            let mut intersect = false;
            for (j, r2) in self.rooms.iter().enumerate() {
                if j == main_room || j == i {
                    continue;
                }

                if ray.intersects_rect(&r2.rect.into_rect()) {
                    intersect = true;
                    break;
                }
            }

            if !intersect && !self.rooms[i].is_linked(main_room) {
                connect_rooms(&mut self.rooms, i, main_room, LinkType::Main);
            }
        }
    }

    pub fn calculate_statistics(&mut self) {
        // Statistical analysis
        let mut most_junctions = 1;

        for r in self.rooms.iter() {
            let len = r.connected().len();
            if  len > most_junctions {
                most_junctions = len;
            }
        }

        for i in 0..self.rooms.len() {
            let junctions = self.rooms[i].connected().len() as f32;
            let focus = inverse_lerp(1., most_junctions as f32, junctions);
            self.rooms[i].value = lerpc(0.,1., focus);
        }
    }

    pub fn generate_portals(&mut self) {
        for i in 0..self.rooms.len() {
            let room = &self.rooms[i];
            let mut intersects = Vec::new();

            for link in room.links() {
                let other = &self.rooms[link.target];
                if link.link_type == LinkType::Direct {
                    // This link requires only a single portal- find the cross-over
                    let west = room.rect.min.x == other.rect.max.x;
                    let east = room.rect.max.x == other.rect.min.x;
                    let north = room.rect.max.y == other.rect.min.y;
                    let south = room.rect.min.y == other.rect.max.y;

                    let line = if north || south {
                        let x_max = room.rect.min.x.max(other.rect.min.x);
                        let x_min = room.rect.max.x.min(other.rect.max.x);
                        let y = if north {
                            room.rect.max.y
                        } else {
                            room.rect.min.y
                        };

                        Line {
                            start: Vec2::new(x_min, y),
                            end: Vec2::new(x_max, y),
                        }
                    } else {
                        let y_max = room.rect.min.y.max(other.rect.min.y);
                        let y_min = room.rect.max.y.min(other.rect.max.y);
                        let x = if east {
                            room.rect.max.x
                        } else if west {
                            room.rect.min.x
                        } else {
                            panic!("cannot generate portal: unable to identify x");
                        };

                        Line {
                            start: Vec2::new(x, y_min),
                            end: Vec2::new(x, y_max),
                        }
                    };

                    let edge = if west {
                        Compass::West
                    } else if east {
                        Compass::East
                    } else if north {
                        Compass::North
                    } else {
                        Compass::South
                    };
                    
                    let intersect: Intersect = Intersect {
                        line,
                    };

                    let intersect_ref = IntersectRef {
                        intersect: 0,
                        other: 
                    }


                    // We now have an edge segment that covers the overlap
                    intersects.push(intersect);
                }
            }

            // Take our cached value and apply
            self.rooms[i].intersects = intersects;
        }
    }

    pub fn export(&self) {
        let mut img = RgbImage::new(self.bounds.max.x as u32 * IMG_SCALE, self.bounds.max.y as u32 * IMG_SCALE);

        for (i, room) in self.rooms.iter().enumerate() {
            let red: u8 = lerpc(0., 255., 1. - room.value) as u8;
            let green: u8 = lerpc(0., 255., room.value) as u8;

            room.draw(&mut img, Rgb::new(red,green,0));

            for j in room.links() {
                if j.target > i {
                    // Only draws the line if target is bigger index. Prevents doubles
                    let a = self.rooms[i].rect;
                    let b = self.rooms[j.target].rect;

                    let line = Line {
                        start: a.center() * IMG_SCALE as f32,
                        end: b.center() * IMG_SCALE as f32,
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
fn module_factory_test() {
    let cfg = ModuleCfg {
        seed: 73563,
        room_count: 12,
        divisions: 32,
        extent: [64., 128.],
        divide_area_chance: 0.1,
        divide_disparity_chance: 0.4,
        split_offset: 0.4,
        split_degredation: 0.98,
    };

    let mut module = ModuleFactory::new(cfg);
    module.link_rooms(true);
    module.generate_islands();
    module.link_islands();
    module.calculate_statistics();
    module.generate_portals();
    module.export();
}