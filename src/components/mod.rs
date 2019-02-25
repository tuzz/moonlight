mod name;
mod transform;
mod image;
mod field_of_view;
mod camera;

pub use name::Name;
pub use transform::Transform;
pub use image::Image;
pub use field_of_view::FieldOfView;
pub use camera::Camera;

use specs::World;

pub fn register_components(world: &mut World) {
    world.register::<Name>();
    world.register::<Transform>();
    world.register::<Image>();
    world.register::<FieldOfView>();
    world.register::<Camera>();
}
