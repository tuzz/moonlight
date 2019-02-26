use cgmath::{Vector2, Vector3, Point3, Matrix4};
use crate::components::{Transform, FieldOfView, Image};
use super::ray::Ray;

pub struct RayGenerator {
    matrix: Matrix4<f64>,
    degrees: Vector2<f64>,
    resolution: Vector2<u32>,

    forward: Vector3<f64>,
    origin: Point3<f64>,

    left_to_right: Vector3<f64>,
    top_to_bottom: Vector3<f64>,
}

impl RayGenerator {
    pub fn new(transform: &Transform, fov: &FieldOfView, image: &Image) -> Self {
        let matrix = transform.matrix;
        let degrees = fov.degrees;
        let resolution = image.resolution;

        let right = matrix.x.truncate();
        let down = -matrix.y.truncate();
        let forward = matrix.z.truncate();
        let origin = Point3::from_homogeneous(matrix.w);

        let left_to_right = right * Self::span(degrees.x);
        let top_to_bottom = down * Self::span(degrees.y);

        Self { matrix, degrees, resolution, forward, origin, left_to_right, top_to_bottom }
    }

    pub fn generate<'a>(&'a self) -> impl Iterator<Item=(Vector2<u32>, Ray)> + 'a {
        let width = self.resolution.x;
        let height = self.resolution.y;

        (0..height).flat_map(move |y| {
            (0..width).map(move |x| {
                let coordinate = Vector2::new(x, y);
                let ray = self.generate_ray(x, y);

                (coordinate, ray)
            })
        })
    }

    fn generate_ray(&self, x: u32, y: u32) -> Ray {
        let ratio = self.pixel_ratio(x, y);
        let vector = self.image_plane_vector(ratio);

        Ray::new(self.origin, vector)
    }

    fn pixel_ratio(&self, x: u32, y: u32) -> Vector2<f64> {
        let x = x as f64 + 0.5;
        let y = y as f64 + 0.5;

        let width = self.resolution.x as f64;
        let height = self.resolution.y as f64;

        Vector2::new(x / width, y / height)
    }

    fn image_plane_vector(&self, pixel_ratio: Vector2<f64>) -> Vector3<f64> {
        let x_offset = pixel_ratio.x - 0.5;
        let y_offset = pixel_ratio.y - 0.5;

        let x = self.left_to_right * x_offset;
        let y = self.top_to_bottom * y_offset;
        let z = self.forward;

        x + y + z
    }

    fn span(degrees: f64) -> f64 {
        let theta = degrees.to_radians() / 2.0;
        let opposite = theta.tan();

        opposite * 2.0
    }
}

#[cfg(test)]
mod test;
