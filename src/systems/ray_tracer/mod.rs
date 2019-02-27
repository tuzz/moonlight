pub mod ray;
pub mod ray_generator;
pub mod intersection;
pub mod intersection_checker;
pub mod illuminator;

use specs::prelude::*;
use crate::components::*;
use cgmath::{Vector3, prelude::InnerSpace};
use ray_generator::RayGenerator;
pub use ray::Ray;
use intersection_checker::IntersectionChecker;
use illuminator::Illuminator;
pub use intersection::Intersection;

pub struct RayTracer;

impl<'a> System<'a> for RayTracer {
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, FieldOfView>,
        WriteStorage<'a, Image>,

        ReadStorage<'a, Shape>,
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
                    .map(|(s, t, m)| (s.shape.intersection(&ray, t), m))
                    .filter_map(|(option, m)| option.map(|i| (i, m)))
                    .min_by(|(a, _), (b, _)| a.cmp(&b));

                // Set the background and continue if the ray doesn't intersect:
                if closest.is_none() {
                    image.set(&coordinate, Vector3::new(0.1, 0.1, 0.1));

                    continue;
                }

                let (intersection, material) = closest.unwrap();

                let total_radiance = (&lights, &transforms).join().map(|(light, transform)| {
                    // Get the amount of light falling on the point of intersection.
                    let (radiance, direction) = Illuminator::radiance_and_direction(
                        &intersection, light, transform
                    );

                    // Build a ray from the point of intersection to the light.
                    let shadow_ray = Ray::new(intersection.origin, direction);

                    // Check if the shadow ray intersects something
                    for (_, t, _) in (&spheres, &transforms, &materials).join() {
                        if let Some(_i) = IntersectionChecker::<Shape>::intersection(&shadow_ray, t) {

                            // TODO: check if the shape is behind the light
                            return 0.0;
                        }
                    }

                    // Calculate the proportion of light falling on the tilted surface.
                    let cosine = direction.dot(intersection.normal).abs();

                    radiance * cosine
                }).sum::<f64>();

                image.set(&coordinate, material.color * total_radiance);
            }
        }
    }
}
