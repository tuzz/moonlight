use specs::prelude::*;
use nalgebra::*;
use std::f64::INFINITY;
use crate::components::{Camera, Transform, FieldOfView, Image, Name};

pub struct SceneGenerator;

impl<'a> System<'a> for SceneGenerator {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, lazy): Self::SystemData) {
        let camera = entities.create();
        let perspective = Self::perspective_matrix(16.0 / 9.0, 0.5, 1.0);
        let degrees = Vector2::new(90.0, 90.0);
        let resolution = Vector2::new(1920, 1080);

        lazy.insert(camera, Camera::new());
        lazy.insert(camera, Transform::new(perspective));
        lazy.insert(camera, FieldOfView::new(degrees));
        lazy.insert(camera, Image::new(resolution));
        lazy.insert(camera, Name::new("camera-1"));
    }
}

impl SceneGenerator {
    fn perspective_matrix(aspect_ratio: f64, field_of_view_y: f64, near_plane: f64) -> Matrix4<f64> {
        Perspective3::new(aspect_ratio, field_of_view_y, near_plane, INFINITY).into_inner()
    }
}

#[cfg(test)]
mod test;
