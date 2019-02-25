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
}

#[cfg(test)]
mod test;
