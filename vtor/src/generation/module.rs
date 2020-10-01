use vek::{Rgb, Vec2};
use prima::geom::{BoundingRect};
use prima::render::{RgbImage, Draw};

const _HOMAN_WIDTH: f32 = 1.2;
const _HOMAN_HEIGHT: f32 = 2.2;
const MODULE_WIDTH: f32 = 64.;
const _MODULE_HEIGHT: f32 = 32.;
const MODULE_DEPTH: f32 = 128.;

const IMAGE_SCALE: u32 = 4;

#[allow(dead_code)]
pub struct Module {
    pub bounds: BoundingRect<f32>,
    pub rooms: Vec<BoundingRect<f32>>,
}

impl Module {
    pub fn new() -> Self {
        let mut bounds = BoundingRect::new_empty(Vec2::zero());
        bounds.max = Vec2::new(MODULE_WIDTH, MODULE_DEPTH);
        bounds.make_valid();

        let mut rooms = vec![bounds.clone()];
        let mut last_index = 0;
        let mut x_axis = false;

        for _ in 0..6 {
            let last_room: BoundingRect<f32> = rooms[last_index].clone();
            rooms.remove(last_index);
            if x_axis {
                let w = (last_room.max.x - last_room.min.x) / 2.;
                let split = last_room.split_at_x(last_room.min.x + w);
                last_index = rooms.len();
                rooms.push(split[0].clone().made_valid());
                rooms.push(split[1].clone().made_valid());
                
                x_axis = false;
            }
            else {
                let h = (last_room.max.y - last_room.min.y) / 2.;
                let split = last_room.split_at_y(last_room.min.y + h);
                last_index = rooms.len();
                rooms.push(split[1].clone().made_valid());
                rooms.push(split[0].clone().made_valid());
                x_axis = true;
            }
        }

        println!("Room count: {}", rooms.len());

        Self {
            bounds,
            rooms,
        }
    }

    pub fn export(&self) {
        let mut img = RgbImage::new(self.bounds.max.x as u32 * IMAGE_SCALE, self.bounds.max.y as u32 * IMAGE_SCALE);

        for i in self.rooms.iter() {
            let boundingbox = BoundingRect {
                min: i.min * IMAGE_SCALE as f32,
                max: i.max * IMAGE_SCALE as f32,
            }.made_valid();

            boundingbox.into_rect().draw(&mut img, Rgb::new(255,0,0));
        }
        img.save("module_export.png").unwrap();
    }
}

#[test]
fn module_test() {
    let module = Module::new();
    module.export();
}