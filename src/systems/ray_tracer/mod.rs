mod ray;
mod ray_generator;
mod intersection;
mod intersection_checker;
mod illuminator;

use specs::prelude::*;
use crate::components::*;
use ray_generator::RayGenerator;
use intersection_checker::IntersectionChecker;
use illuminator::Illuminator;

pub struct RayTracer;

impl<'a> System<'a> for RayTracer {
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, FieldOfView>,
        WriteStorage<'a, Image>,

        ReadStorage<'a, Sphere>,
        ReadStorage<'a, Material>,
    );

    fn run(&mut self, (cameras, transforms, fovs, mut images, spheres, materials): Self::SystemData) {
        for (_, transform, fov, image) in (&cameras, &transforms, &fovs, &mut images).join() {
            let ray_generator = RayGenerator::new(transform, fov, image);

            // For each ray that goes through a pixel in the image plane:
            for (coordinate, ray) in ray_generator.generate() {

                // Get the closest intersection/material (if any) for the ray:
                let closest = (&spheres, &transforms, &materials).join()
                    .map(|(_, t, m)| (IntersectionChecker::<Sphere>::intersection(&ray, t), m))
                    .filter_map(|(option, m)| option.map(|i| (i, m)))
                    .min_by(|(a, _), (b, _)| a.cmp(&b));

                // Continue if the ray doesn't intersect:
                if closest.is_none() {
                    continue;
                }

                let (intersection, material) = closest.unwrap();

                // Temporarily set the color based on the surface normal:
                image.set(&coordinate, intersection.normal);
            }
        }
    }
}
