mod ray;
mod ray_generator;
mod intersection;
mod intersection_checker;
mod illuminator;

use specs::prelude::*;
use crate::components::*;
use cgmath::prelude::InnerSpace;
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
        ReadStorage<'a, Light>,
    );

    fn run(&mut self, (cameras, transforms, fovs, mut images, spheres, materials, lights): Self::SystemData) {
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

                let total_radiance = (&lights, &transforms).join().map(|(light, transform)| {
                    // Get the amount of light falling on the point of intersection.
                    let (radiance, direction) = Illuminator::radiance_and_direction(
                        &intersection, light, transform
                    );

                    // Calculate the proportion of light falling on the tilted surface.
                    let cosine = direction.dot(intersection.normal).abs();

                    radiance * cosine
                }).sum::<f64>();

                image.set(&coordinate, material.color * total_radiance);
            }
        }
    }
}
