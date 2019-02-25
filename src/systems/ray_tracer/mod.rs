mod ray;
mod ray_generator;

use specs::prelude::*;
use cgmath::{Vector2, Vector3};
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
            let _ray_generator = RayGenerator::new(transform, fov, image);

            // For now, just try writing to the camera image data.
            for i in 0..100 {
                let coordinate = Vector2::new(i, i);
                let color = Vector3::new(0.5, 0.0, 0.5);

                image.set(&coordinate, color);
            }
        }
    }
}
