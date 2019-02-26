use specs::prelude::*;

#[derive(Component)]
pub struct Frame {
    pub number: u32,
}

impl Frame {
    pub fn new(number: u32) -> Self {
        Self { number }
    }
}
