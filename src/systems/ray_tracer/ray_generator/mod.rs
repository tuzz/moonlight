use cgmath::{Vector2, Vector3, Matrix4};
use crate::components::{Transform, FieldOfView, Image};

pub struct RayGenerator {
    matrix: Matrix4<f64>,
    degrees: Vector2<f64>,
    resolution: Vector2<u32>,

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

        let left_to_right = right * Self::span(degrees.x);
        let top_to_bottom = down * Self::span(degrees.y);

        Self { matrix, degrees, resolution, left_to_right, top_to_bottom }
    }

    fn pixel_ratio(&self, x: usize, y: usize) -> Vector2<f64> {
        let x = x as f64 + 0.5;
        let y = y as f64 + 0.5;

        let width = self.resolution.x as f64;
        let height = self.resolution.y as f64;

        Vector2::new(x / width, y / height)
    }

    fn span(degrees: f64) -> f64 {
        let theta = degrees.to_radians() / 2.0;
        let opposite = theta.tan();

        opposite * 2.0
    }
}

#[cfg(test)]
mod test;
