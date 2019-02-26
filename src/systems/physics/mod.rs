use specs::prelude::*;
use cgmath::Matrix4;
use crate::components::{Transform, Velocity};

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Velocity>
    );

    fn run(&mut self, (mut transforms, velocities): Self::SystemData) {
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            let translation = Matrix4::from_translation(velocity.vector);

            transform.matrix = transform.matrix * translation;
        }
    }
}
