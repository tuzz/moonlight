use cgmath::{Point3, Vector3, prelude::InnerSpace];
use std::marker::PhantomData;
use super::intersection::Intersection;
use super::ray::Ray;
use crate::components::{Shape, Transform};

pub struct IntersectionChecker<T> {
    _t: PhantomData<T>,
}

const EPSILON: f64 = 0.0000000001;

impl IntersectionChecker<Shape> {
    pub fn intersection(ray: &Ray, transform: &Transform) -> Option<Intersection> {
        let ray = Self::to_object_space(ray, transform);
        let radius = 1.0;

        // Calculate the vector from the center of the sphere to the ray's origin.
        let center_to_ray = ray.origin - Point3::new(0.0, 0.0, 0.0);

        // Calculate the coefficients of the quadratic rearrangement:
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * center_to_ray.dot(ray.direction);
        let c = center_to_ray.dot(center_to_ray) - radius * radius;

        // Calculate the term under the square root of the quadratic equation:
        let discriminant = b * b - 4.0 * a * c;

        // If the discriminant is negative, there are no intersections:
        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();

        // Find both solutions of the quadratic equation:
        let t0 = (-b - sqrt) / (a * 2.0);
        let t1 = (-b + sqrt) / (a * 2.0);

        if t0 <= EPSILON && t1 <= EPSILON {
            return None;
        }

        // The intersection is the smallest positive solution:
        let ray_t = if t0 > EPSILON { t0 } else { t1 };
        let origin = ray.at(ray_t);
        let normal = origin - Point3::new(0.0, 0.0, 0.0);

        let origin = Self::point_to_world_space(&origin, transform);
        let normal = Self::vector_to_world_space(&normal, transform);

        Some(Intersection::new(ray_t, origin, normal))
    }

    fn to_object_space(ray: &Ray, transform: &Transform) -> Ray {
        let world_to_object = transform.inverse;

        let origin = world_to_object * ray.origin.to_homogeneous();
        let direction = world_to_object * ray.direction.extend(0.0);

        Ray::new(Point3::from_homogeneous(origin), direction.truncate())
    }

    fn vector_to_world_space(vector: &Vector3<f64>, transform: &Transform) -> Vector3<f64> {
        let object_to_world = transform.matrix;
        let vector4 = object_to_world * vector.extend(0.0);

        vector4.truncate()
    }

    fn point_to_world_space(point: &Point3<f64>, transform: &Transform) -> Point3<f64> {
        let object_to_world = transform.matrix;
        let point4 = object_to_world * point.to_homogeneous();

        Point3::from_homogeneous(point4)
    }
}

#[cfg(test)]
mod test;
