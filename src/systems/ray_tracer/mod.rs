mod ray;
use specs::prelude::*;
use nalgebra::*;
use crate::components::{Camera, Transform, Image};

pub struct RayTracer;

impl<'a> System<'a> for RayTracer {
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Image>,
    );

    fn run(&mut self, (camera, transform, mut image): Self::SystemData) {
        for (_, _, image) in (&camera, &transform, &mut image).join() {

            // For now, just try writing to the camera image data.
            let (_, min) = image.resolution.argmin();

            for i in 0..min {
                let coordinate = Vector2::new(i, i);
                let color = Vector3::new(0.5, 0.0, 0.5);

                image.set(&coordinate, color);
            }
        }
    }
}
