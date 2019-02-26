use specs::prelude::*;
use cgmath::Vector3;

#[derive(Component)]
pub struct Velocity {
    pub vector: Vector3<f64>,
}

impl Velocity {
    pub fn new(vector: Vector3<f64>) -> Self {
        Self { vector }
    }
}
