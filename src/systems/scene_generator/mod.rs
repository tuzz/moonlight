use specs::prelude::*;
use cgmath::{Vector2, Vector3, Matrix4};
use crate::components::*;

pub struct SceneGenerator;

impl<'a> System<'a> for SceneGenerator {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, lazy): Self::SystemData) {
        let camera = entities.create();
        lazy.insert(camera, Camera::new());
        lazy.insert(camera, Self::transform(1.0, Vector3::new(0.0, 0.0, -20.0)));
        lazy.insert(camera, FieldOfView::new(Vector2::new(86.0, 55.36)));
        lazy.insert(camera, Image::new(Vector2::new(1920, 1080)));
        lazy.insert(camera, Velocity::new(Vector3::new(0.0, 0.0, 0.05)));
        lazy.insert(camera, Frame::new(1));
        lazy.insert(camera, Name::new("camera-1"));

        let sphere1 = entities.create();
        lazy.insert(sphere1, Sphere::new());
        lazy.insert(sphere1, Self::transform(2.0, Vector3::new(-1.0, 4.0, 15.0)));
        lazy.insert(sphere1, Material::new(Vector3::new(1.0, 0.2, 0.2)));
        lazy.insert(sphere1, Name::new("sphere-1"));

        let sphere2 = entities.create();
        lazy.insert(sphere2, Sphere::new());
        lazy.insert(sphere2, Self::transform(5.0, Vector3::new(2.0, 2.0, 20.0)));
        lazy.insert(sphere2, Material::new(Vector3::new(0.2, 1.0, 0.2)));
        lazy.insert(sphere2, Name::new("sphere-2"));

        let sphere3 = entities.create();
        lazy.insert(sphere3, Sphere::new());
        lazy.insert(sphere3, Self::transform(3.0, Vector3::new(10.0, -1.0, 25.0)));
        lazy.insert(sphere3, Material::new(Vector3::new(0.5, 0.0, 0.7)));
        lazy.insert(sphere3, Name::new("sphere-3"));

        let sphere4 = entities.create();
        lazy.insert(sphere4, Sphere::new());
        lazy.insert(sphere4, Self::transform(2.0, Vector3::new(12.0, 4.0, 24.0)));
        lazy.insert(sphere4, Material::new(Vector3::new(1.0, 1.0, 0.2)));
        lazy.insert(sphere4, Name::new("sphere-4"));

        let sphere5 = entities.create();
        lazy.insert(sphere5, Sphere::new());
        lazy.insert(sphere5, Self::transform(3.0, Vector3::new(-5.0, -2.0, 12.0)));
        lazy.insert(sphere5, Material::new(Vector3::new(0.0, 0.2, 0.8)));
        lazy.insert(sphere5, Name::new("sphere-5"));

        let sphere6 = entities.create();
        lazy.insert(sphere6, Sphere::new());
        lazy.insert(sphere6, Self::transform(1.0, Vector3::new(-1.0, -1.0, 11.0)));
        lazy.insert(sphere6, Material::new(Vector3::new(0.8, 0.5, 0.7)));
        lazy.insert(sphere6, Name::new("sphere-6"));

        let sphere7 = entities.create();
        lazy.insert(sphere7, Sphere::new());
        lazy.insert(sphere7, Self::transform(4.0, Vector3::new(-11.0, 6.0, 12.0)));
        lazy.insert(sphere7, Material::new(Vector3::new(1.0, 1.0, 1.0)));
        lazy.insert(sphere7, Name::new("sphere-7"));

        let sphere8 = entities.create();
        lazy.insert(sphere8, Sphere::new());
        lazy.insert(sphere8, Self::transform(5.0, Vector3::new(6.0, -9.0, 12.0)));
        lazy.insert(sphere8, Material::new(Vector3::new(0.0, 0.0, 0.0)));
        lazy.insert(sphere8, Name::new("sphere-8"));

        let light1 = entities.create();
        lazy.insert(light1, Light::new(300.0));
        lazy.insert(light1, Self::transform(1.0, Vector3::new(-3.0, 12.0, -2.0)));
        lazy.insert(light1, Name::new("light-1"));

        let light2 = entities.create();
        lazy.insert(light2, Light::new(100.0));
        lazy.insert(light2, Self::transform(1.0, Vector3::new(12.0, 12.0, 22.0)));
        lazy.insert(light2, Name::new("light-2"));

        let light3 = entities.create();
        lazy.insert(light3, Light::new(200.0));
        lazy.insert(light3, Self::transform(1.0, Vector3::new(-5.0, 8.0, 30.0)));
        lazy.insert(light3, Name::new("light-3"));
    }
}

impl SceneGenerator {
    fn transform(scale: f64, translate: Vector3<f64>) -> Transform {
        let translation = Matrix4::from_translation(translate);
        let scale = Matrix4::from_scale(scale);

        Transform::new(translation * scale)
    }
}

#[cfg(test)]
mod test;
