use super::*;

#[allow(unused_imports)]
use cgmath::SquareMatrix;

type Subject = RayGenerator;

fn subject() -> Subject {
    let transform = Transform::new(Matrix4::identity());
    let fov = FieldOfView::new(Vector2::new(90.0, 90.0));
    let image = Image::new(Vector2::new(20, 10));

    Subject::new(&transform, &fov, &image)
}

mod new {
    use super::*;

    #[test]
    fn it_builds_the_generator_from_the_camera_related_components() {
        let subject = subject();

        assert_eq!(subject.matrix, Matrix4::identity());
        assert_eq!(subject.degrees, Vector2::new(90.0, 90.0));
        assert_eq!(subject.resolution, Vector2::new(20, 10));
    }

    #[test]
    fn it_sets_vectors_that_span_left_to_right_and_top_to_bottom() {
        let subject = subject();

        assert_approx_eq!(subject.left_to_right.x, 2.0);
        assert_approx_eq!(subject.top_to_bottom.y, -2.0);
    }

    #[test]
    fn it_sets_a_vector_from_the_eye_to_the_center_of_the_image_plane() {
        let subject = subject();

        assert_eq!(subject.forward, Vector3::new(0.0, 0.0, 1.0));
    }
}

mod image_plane_vector {
    use super::*;

    #[test]
    fn it_returns_a_vector_from_the_eye_to_the_pixel_on_the_image_plane() {
        let subject = subject();

        let result = subject.image_plane_vector(Vector2::new(0.5, 0.5));
        assert_eq!(result, Vector3::new(0.0, 0.0, 1.0));

        let result = subject.image_plane_vector(Vector2::new(0.1, 0.2));
        assert_approx_eq!(result.x, -0.8);
        assert_approx_eq!(result.y, 0.6);

        let result = subject.image_plane_vector(Vector2::new(0.4, 0.6));
        assert_approx_eq!(result.x, -0.2);
        assert_approx_eq!(result.y, -0.2);
    }
}

mod pixel_ratio {
    use super::*;

    #[test]
    fn it_returns_the_ratio_of_the_pixel_coordinate_to_the_image_resolution() {
        let subject = subject();

        assert_eq!(subject.pixel_ratio(0, 0), Vector2::new(0.025, 0.05));
        assert_eq!(subject.pixel_ratio(2, 4), Vector2::new(0.125, 0.45));
        assert_eq!(subject.pixel_ratio(17, 7), Vector2::new(0.875, 0.75));
    }
}

mod span {
    use super::*;

    #[test]
    fn it_returns_the_width_or_height_of_the_image_plane_located_a_unit_distance_away() {
        assert_approx_eq!(Subject::span(0.0), 0.0);
        assert_approx_eq!(Subject::span(90.0), 2.0);
    }
}
