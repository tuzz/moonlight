mod name;

pub use name::Name;

use specs::World;

pub fn register_components(world: &mut World) {
    world.register::<Name>();
}
