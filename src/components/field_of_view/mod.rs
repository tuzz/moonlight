use specs::prelude::*;
use cgmath::Vector2;

#[derive(Component)]
pub struct FieldOfView {
    pub degrees: Vector2<f64>,
}

impl FieldOfView {
    pub fn new(degrees: Vector2<f64>) -> Self {
        Self { degrees }
    }
}
