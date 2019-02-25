mod name;
mod transform;
mod image;

pub use name::Name;
pub use transform::Transform;
pub use image::Image;

use specs::World;

pub fn register_components(world: &mut World) {
    world.register::<Name>();
    world.register::<Transform>();
    world.register::<Image>();
}
