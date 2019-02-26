use super::*;

type Subject = Intersection;

mod new {
    use super::*;

    #[test]
    fn it_builds_an_intersection_with_a_rays_parameter_and_surface_normal() {
        let ray_t = 1.23;
        let origin = Point3::new(0.1, 0.2, 0.3);
        let normal = Vector3::new(1.0, 0.0, 0.0);

        let subject = Subject::new(ray_t, origin, normal);

        assert_eq!(subject.ray_t, ray_t);
        assert_eq!(subject.origin, origin);
        assert_eq!(subject.normal, normal);
    }

    #[test]
    fn it_normalizes_the_surface_normal() {
        let origin = Point3::new(0.1, 0.2, 0.3);
        let normal = Vector3::new(2.0, 0.0, 0.0);
        let subject = Subject::new(1.23, origin, normal);

        assert_eq!(subject.normal, Vector3::new(1.0, 0.0, 0.0));
    }
}
