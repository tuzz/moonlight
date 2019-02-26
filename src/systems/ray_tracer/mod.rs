mod ray;
mod ray_generator;

use specs::prelude::*;
use crate::components::{Camera, Transform, FieldOfView, Image};
use ray_generator::RayGenerator;

pub struct RayTracer;

impl<'a> System<'a> for RayTracer {
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, FieldOfView>,
        WriteStorage<'a, Image>,
    );

    fn run(&mut self, (camera, transform, fov, mut image): Self::SystemData) {
        for (_, transform, fov, image) in (&camera, &transform, &fov, &mut image).join() {
            let ray_generator = RayGenerator::new(transform, fov, image);

            for (coordinate, ray) in ray_generator.generate() {
                image.set(&coordinate, ray.direction);
            }
        }
    }
}
