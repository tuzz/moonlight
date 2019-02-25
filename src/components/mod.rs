mod name;
mod transform;

pub use name::Name;
pub use transform::Transform;

use specs::World;

pub fn register_components(world: &mut World) {
    world.register::<Name>();
    world.register::<Transform>();
}
