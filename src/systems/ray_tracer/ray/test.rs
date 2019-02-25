use super::*;

type Subject = Ray;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_ray_with_origin_and_direction() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);

        let subject = Subject::new(origin, direction);

        assert_eq!(subject.origin, origin);
        assert_eq!(subject.direction, direction);
    }
}

mod at {
    use super::*;

    #[test]
    fn it_returns_the_point_at_a_position_along_the_ray() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);

        let subject = Subject::new(origin, direction);

        assert_eq!(subject.at(0.5), Point3::new(3.0, 4.5, 6.0));
        assert_eq!(subject.at(1.0), Point3::new(5.0, 7.0, 9.0));
        assert_eq!(subject.at(1.5), Point3::new(7.0, 9.5, 12.0));
    }
}
