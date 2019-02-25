use specs::prelude::*;
use nalgebra::Matrix4;
use std::ops::Deref;

#[derive(Component, Clone)]
pub struct Transform {
    pub matrix: Matrix4<f64>,
}

impl Transform {
    pub fn new(matrix: Matrix4<f64>) -> Self {
        Self { matrix }
    }
}

impl Deref for Transform {
    type Target = Matrix4<f64>;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

#[cfg(test)]
mod test;
