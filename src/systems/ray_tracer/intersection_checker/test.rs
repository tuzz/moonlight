//use super::*;
//use cgmath::{Point3, Vector3, Matrix4};
//
//type Subject<T> = IntersectionChecker<T>;
//
//mod intersection {
//    use super::*;
//
//    fn intersection(ray_y_direction: f64, sphere_radius: f64) -> Option<Intersection> {
//        let origin = Point3::new(0.0, 0.0, 0.0);
//        let direction = Vector3::new(1.0, ray_y_direction, 0.0);
//        let ray = Ray::new(origin, direction);
//
//        let scale = Matrix4::from_scale(sphere_radius);
//        let translation = Matrix4::from_translation(Vector3::new(2.0, 0.0, 0.0));
//        let transform = Transform::new(translation * scale);
//
//        Subject::intersection(&ray, &transform)
//    }
//
//    #[test]
//    fn it_returns_none_if_the_ray_does_not_intersect_the_sphere() {
//        assert_eq!(intersection(0.6, 1.0), None);
//    }
//
//    #[test]
//    fn it_returns_the_nearest_intersection_if_the_ray_intersects_the_sphere() {
//        let intersection = intersection(0.5, 1.0)
//            .expect("The ray should have intersected the front of the sphere.");
//
//        assert_eq!(intersection.ray_t, 1.2);
//        assert_eq!(intersection.normal, Vector3::new(-0.8, 0.6, 0.0));
//    }
//
//    #[test]
//    fn it_ignores_intersections_at_the_origin_of_the_ray() {
//        let intersection = intersection(0.5, 2.0)
//            .expect("The ray should have intersected the back of the sphere.");
//
//        assert_eq!(intersection.ray_t, 3.2);
//    }
//
//    #[test]
//    fn it_ignores_intersections_epsilon_along_the_ray_to_counter_floating_precision_errors() {
//        let intersection = intersection(0.5, 1.99999999999)
//            .expect("The ray should have intersected the back of the sphere.");
//
//        assert_eq!(intersection.ray_t > 3.1, true);
//        assert_eq!(intersection.ray_t < 3.3, true);
//    }
//}
