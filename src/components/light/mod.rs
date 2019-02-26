use specs::prelude::*;

#[derive(Component)]
pub struct Light {
    pub power: f64,
}

impl Light {
    pub fn new(power: f64) -> Self {
        Self { power }
    }
}
