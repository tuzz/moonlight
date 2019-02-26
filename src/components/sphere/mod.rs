use specs::prelude::*;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Sphere;

impl Sphere {
    pub fn new() -> Self {
        Self { }
    }
}
