mod ray;
mod ray_generator;
mod intersection;
mod intersection_checker;

use specs::prelude::*;
use cgmath::Vector3;
use crate::components::*;
use ray_generator::RayGenerator;
use intersection_checker::IntersectionChecker;

pub struct RayTracer;

impl<'a> System<'a> for RayTracer {
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, FieldOfView>,
        WriteStorage<'a, Image>,

        ReadStorage<'a, Sphere>,
    );

    fn run(&mut self, (cameras, transforms, fovs, mut images, spheres): Self::SystemData) {
        for (_, transform, fov, image) in (&cameras, &transforms, &fovs, &mut images).join() {
            let ray_generator = RayGenerator::new(transform, fov, image);

            for (coordinate, ray) in ray_generator.generate() {
                for (_, transform) in (&spheres, &transforms).join() {
                    if let Some(_) = IntersectionChecker::<Sphere>::intersection(&ray, transform) {
                        image.set(&coordinate, Vector3::new(1.0, 0.0, 0.0));
                    }
                }
            }
        }
    }
}
