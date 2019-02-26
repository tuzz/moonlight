use specs::prelude::*;
use cgmath::{Vector2, Vector3, Matrix4, SquareMatrix};
use crate::components::{Camera, Transform, FieldOfView, Image, Name, Sphere};

pub struct SceneGenerator;

impl<'a> System<'a> for SceneGenerator {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, lazy): Self::SystemData) {
        let camera = entities.create();

        lazy.insert(camera, Camera::new());
        lazy.insert(camera, Transform::new(Matrix4::identity()));
        lazy.insert(camera, FieldOfView::new(Vector2::new(86.0, 55.36)));
        lazy.insert(camera, Image::new(Vector2::new(1920, 1080)));
        lazy.insert(camera, Name::new("camera-1"));

        let sphere = entities.create();

        lazy.insert(sphere, Sphere::new());
        lazy.insert(sphere, Transform::new(Matrix4::from_translation(Vector3::new(0.0, 0.0, 5.0))));
        lazy.insert(sphere, Name::new("sphere-1"));
    }
}

#[cfg(test)]
mod test;
