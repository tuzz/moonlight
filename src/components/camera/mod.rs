use specs::prelude::*;

#[derive(Component)]
pub struct Camera;

impl Camera {
    pub fn new() -> Self {
        Self { }
    }
}
