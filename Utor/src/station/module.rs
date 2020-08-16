use vek::{Vec2, Vec3};
use rand::prelude::*;
use corale::core::*;
use corale::mesh::*;
use corale::grid::*;
use corale::geom::*;
use crate::primatives::*;
use crate::station::{room_factory::*, room::Room};
use crate::torus_modifier::TorusModifier;
use crate::GrowableBox;

const LENGTH_K: Float = 3.;
const WIDTH_K: Float = 1.;
const HEIGHT_K: Float = 1.;

const AREA_INNER: (Float,Float,Float) = (0.5, 0.5, 1.);
const GRID_SIZE: (Int,Int,Int) = (8,8,32);

const ROOM_COUNT: usize = 10;
const ROOM_SIZE_LIMMIT: usize = 3;


const ROOM_FILL: usize = 65;
const ROOM_SMOOTHING: usize = 2;
const ROOM_NEIGHBOR_THRESHHOLD: usize = 3;

type Float = f64;
type Int = i64;

#[derive(Clone)]
pub struct Module {
    index: usize,
    torus_mod: TorusModifier<Float>,
    bounds: BoundingBox<Int>, 
    mesh: Mesh<Float>,
    rooms: Vec<Room<Int>>,
}


impl Module {
    /// Creates a new module. 
    /// theta: the angle size of the segment's arc
    /// radius: the radius from the ring center to the middle of the module
    /// mesh: parent mesh to clone from. 
    pub fn new(index: usize, arc: Float, radius: Float, mesh: &Mesh<Float>) -> Self {
        let angle = index as Float * arc;
        let mut torus_mod = TorusModifier::new(Vec2::zero(), angle, radius, arc, 0., 0.);
        let k = torus_mod.length() / LENGTH_K;
        torus_mod.set_size(k * WIDTH_K, k * HEIGHT_K);
        let mut new_mesh = mesh.clone();
        let name = format!("module_{}", index);
        new_mesh.set_name(name);

        let mut me = Self {
            index,
            rooms: Vec::new(),
            torus_mod,
            bounds: BoundingBox::new(Vec3::zero(), Vec3::new(GRID_SIZE.0, GRID_SIZE.1, GRID_SIZE.2)),
            mesh: new_mesh,
        };

        me.generate_rooms_cellular(ROOM_FILL);
        me
    }

    /// Builds a room in every single tile. Debug use only!
    pub fn generate_all_rooms(&mut self) {
        for x in 0..GRID_SIZE.0 {
            for y in 0..GRID_SIZE.1 {
                for z in 0..GRID_SIZE.2 {
                    let pos: Vec3<Int> = Vec3::new(x, y, z);
                    let new_room = Room::new(GrowableBox::new(pos));
                    self.rooms.push(new_room);
                }
            }
        }
    }

    /// A directional approach to gennerating rooms
    pub fn generate_random_rooms(&mut self) {
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
            let pos: Vec3<Int> = Vec3::new(x, y, z);

            let room = build_room(pos, &self.bounds, &self.rooms, RoomFactory::Marching(ROOM_SIZE_LIMMIT));
            if room.is_some() {
                rooms_left -= 1;
                self.rooms.push(room.unwrap());
            }
        }
    }

    pub fn generate_rooms_cellular(&mut self, percent_fill: usize) {
        let fill = (maths::clamp(percent_fill, 0, 100)) as Int;
        let mut rng = thread_rng();
        let mut grid = GridMap::<bool, Int>::new(Vec3::zero(), Vec3::new(GRID_SIZE.0, GRID_SIZE.1, GRID_SIZE.2));
        for x in 0..GRID_SIZE.0 {
            for y in 0..GRID_SIZE.1 {
                for z in 0..GRID_SIZE.2 {
                    let pos: Vec3<Int> = Vec3::new(x, y, z);
                    let number = rng.gen_range(0, 100);
                    let is_room = number < fill;
                    if is_room {
                        let _ = grid.add(is_room, pos).unwrap();
                    }
                }
            }
        }

        //Smooting
        for _ in 0..ROOM_SMOOTHING {
            grid = self.smooth_grid(grid, ROOM_NEIGHBOR_THRESHHOLD);
        }

        grid = self.smooth_grid(grid, 1);

        for object in grid.items().iter() {
            if *object.item() {
                let new_room = Room::new(GrowableBox::new(object.position()));
                self.rooms.push(new_room);
            }
        }
    }

    fn smooth_grid(&mut self, grid: GridMap<bool, Int>, amount: usize) -> GridMap<bool, Int> {

        let mut new_grid = GridMap::<bool, Int>::from_boundingbox(self.bounds);

        for grid_object in grid.items() {
            let is_room = grid_object.item();
            let pos = grid_object.position();
            if *is_room {
                if grid.neighbors(pos, false).len() > amount {
                    new_grid.add(true, pos).unwrap();
                }
            }
        }
        new_grid
    }

    fn rooms_to_mesh(&self, mesh: &mut Mesh<Float>) {
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
            //println!("Building Room {} -> {}...", room.min(), room.max());
            make_box(pos_a, pos_b, mesh);
        }
    }

    pub fn mesh(&self) -> &Mesh<Float> {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh<Float> {
        &mut self.mesh
    }

    pub fn build(&self) -> Mesh<Float> {
        let mut mesh = self.mesh.clone();
        self.rooms_to_mesh(&mut mesh);
        self.torus_mod.apply(&mut mesh);
        mesh.invert_z();
        mesh
    }

    pub fn index(&self) -> usize {
        self.index
    }

}



#[cfg(test)]
mod tests {
    use crate::station::module::Module;
    use corale::mesh::Mesh;
    use corale::wavefront::*;
    use std::io::BufReader;
    use std::fs::File;
    
    #[test]
    fn module_test() {
        let angle = 2. * std::f64::consts::PI / 6. / 3.;
        
        let file = File::open(format!("assets/module.obj")).unwrap();
        let input = BufReader::new(file);
        let mesh: Mesh<f64> = parse(input).unwrap();
        let module = Module::new(0, angle, 800., &mesh);
        let build = module.build();
        let file_name = "../bin/renders/a_test".to_string();
        export(&build, file_name).unwrap();
    }
}