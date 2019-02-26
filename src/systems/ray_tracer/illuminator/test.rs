use super::*;
use cgmath::{Point3, Matrix4};

type Subject = Illuminator;

mod radiance_and_direction {
    use super::*;

    #[test]
    fn it_builds_a_vector_pointing_towards_the_light() {
        let origin = Point3::new(4.0, 5.0, 6.0);
        let intersection = Intersection::new(0.0, origin, Vector3::new(0.0, 0.0, 0.0));

        let light = Light::new(5.0);

        let translation = Matrix4::from_translation(Vector3::new(1.0, 2.0, 3.0));
        let transform = Transform::new(translation);

        let (_, direction) = Subject::radiance_and_direction(&intersection, &light, &transform);

        assert_eq!(direction, Vector3::new(-3.0, -3.0, -3.0).normalize());
    }

    #[test]
    fn it_calculates_radiance_by_dividing_the_lights_power_by_the_radius_squared() {
        let origin = Point3::new(4.0, 5.0, 6.0);
        let intersection = Intersection::new(0.0, origin, Vector3::new(0.0, 0.0, 0.0));

        let light = Light::new(5.0);

        let translation = Matrix4::from_translation(Vector3::new(1.0, 2.0, 3.0));
        let transform = Transform::new(translation);

        let (radiance, _) = Subject::radiance_and_direction(&intersection, &light, &transform);

        assert_eq!(radiance, 5.0 / 27.0);
    }
}
