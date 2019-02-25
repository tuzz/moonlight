use specs::prelude::*;
use nalgebra::geometry::Transform3;
use std::ops::Deref;

#[derive(Component)]
pub struct Transform {
    pub inner: Transform3<f64>,
}

impl Transform {
    pub fn new(inner: Transform3<f64>) -> Self {
        Self { inner }
    }
}

impl Deref for Transform {
    type Target = Transform3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod test;
