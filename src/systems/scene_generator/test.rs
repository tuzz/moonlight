use super::*;
use crate::systems::TestHelper;

type Subject = SceneGenerator;

mod run {
    use super::*;

    #[test]
    fn it_builds_a_camera() {
        let mut t = TestHelper::new();

        t.run(SceneGenerator);

        let camera = t.entity("camera-1");

        assert_eq!(t.has::<Camera>(camera), true);
        assert_eq!(t.has::<Transform>(camera), true);
        assert_eq!(t.has::<FieldOfView>(camera), true);
        assert_eq!(t.has::<Image>(camera), true);
    }

    #[test]
    fn it_builds_a_sphere() {
        let mut t = TestHelper::new();

        t.run(SceneGenerator);

        let sphere = t.entity("sphere-1");

        assert_eq!(t.has::<Sphere>(sphere), true);
        assert_eq!(t.has::<Transform>(sphere), true);
        assert_eq!(t.has::<Material>(sphere), true);
    }
}
