use cgmath::{Vector2, Matrix4};
use crate::components::{Transform, FieldOfView, Image};

pub struct RayGenerator {
    matrix: Matrix4<f64>,
    degrees: Vector2<f64>,
    resolution: Vector2<u32>,
}

impl RayGenerator {
    pub fn new(transform: &Transform, fov: &FieldOfView, image: &Image) -> Self {
        let matrix = transform.matrix;
        let degrees = fov.degrees;
        let resolution = image.resolution;

        Self { matrix, degrees, resolution }
    }

    fn pixel_ratio(&self, x: usize, y: usize) -> Vector2<f64> {
        let x = x as f64 + 0.5;
        let y = y as f64 + 0.5;

        let width = self.resolution.x as f64;
        let height = self.resolution.y as f64;

        Vector2::new(x / width, y / height)
    }
}

#[cfg(test)]
mod test;
