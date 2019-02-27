use crate::systems::ray_tracer::{Ray, Intersection};
use crate::components::Transform;
use specs::prelude::*;

#[derive(Component)]
pub struct Shape {
    pub shape: Box<ShapeTrait>,
}

impl Shape {
    pub fn new<T: ShapeTrait>(shape: T) -> Self {
        Self { shape: Box::new(shape) }
    }
}

pub trait ShapeTrait: Send + Sync + 'static {
    fn intersection(&self, ray: &Ray, transform: &Transform) -> Option<Intersection>;
}
