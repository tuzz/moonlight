use cgmath::{Point3, Vector3, prelude::InnerSpace};
use crate::components::{Light, Transform};
use super::intersection::Intersection;

pub struct Illuminator;

impl Illuminator {
    pub fn radiance_and_direction(intersection: &Intersection, light: &Light, transform: &Transform) -> (f64, Vector3<f64>) {
        let origin = Self::to_object_space(intersection.origin, transform);
        let direction = Point3::new(0.0, 0.0, 0.0) - origin;

        let radius = direction.magnitude();
        let radiance = light.power / radius.powf(2.0);

        let direction = Self::to_world_space(direction, transform).normalize();

        (radiance, direction)
    }

    fn to_object_space(point: Point3<f64>, transform: &Transform) -> Point3<f64> {
        let world_to_object = transform.inverse;
        let point4 = world_to_object * point.to_homogeneous();

        Point3::from_homogeneous(point4)
    }

    fn to_world_space(vector: Vector3<f64>, transform: &Transform) -> Vector3<f64> {
        let object_to_world = transform.matrix;
        let vector4 = object_to_world * vector.extend(0.0);

        vector4.truncate()
    }
}

#[cfg(test)]
mod test;
