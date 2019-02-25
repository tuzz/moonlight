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

mod pixel_ratio {
    use super::*;

    #[test]
    fn it_returns_the_ratio_of_the_pixel_coordinate_to_the_image_resolution() {
        let transform = Transform::new(Matrix4::identity());
        let fov = FieldOfView::new(Vector2::new(90.0, 90.0));
        let image = Image::new(Vector2::new(20, 10));

        let subject = Subject::new(&transform, &fov, &image);

        assert_eq!(subject.pixel_ratio(0, 0), Vector2::new(0.025, 0.05));
        assert_eq!(subject.pixel_ratio(2, 4), Vector2::new(0.125, 0.45));
        assert_eq!(subject.pixel_ratio(17, 7), Vector2::new(0.875, 0.75));
    }
}
