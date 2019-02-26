use specs::prelude::*;
use cgmath::Vector3;

#[derive(Component)]
pub struct Material {
    pub color: Vector3<f64>,
}

impl Material {
    pub fn new(color: Vector3<f64>) -> Self {
        Self { color }
    }
}
