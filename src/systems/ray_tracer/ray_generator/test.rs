use super::*;

#[allow(unused_imports)]
use cgmath::SquareMatrix;

type Subject = RayGenerator;

mod new {
    use super::*;

    #[test]
    fn it_builds_the_generator_from_the_camera_related_components() {
        let transform = Transform::new(Matrix4::identity());
        let fov = FieldOfView::new(Vector2::new(90.0, 90.0));
        let image = Image::new(Vector2::new(20, 10));

        let subject = Subject::new(&transform, &fov, &image);

        assert_eq!(subject.matrix, Matrix4::identity());
        assert_eq!(subject.degrees, Vector2::new(90.0, 90.0));
        assert_eq!(subject.resolution, Vector2::new(20, 10));
    }
}
