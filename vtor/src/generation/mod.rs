mod module_factory;
mod room;
mod link;
mod portal;
pub mod intersect;

pub use room::Room;
pub use portal::Portal;
pub use link::{Link, LinkType};
pub use module_factory::ModuleFactory;

pub const IMG_SCALE: u32 = 4;