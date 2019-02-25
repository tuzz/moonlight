use super::*;
use crate::systems::TestHelper;

type Subject = SceneGenerator;

mod run {
    use super::*;

    #[test]
    fn it_builds_a_camera_with_a_transform_and_an_image() {
        let mut t = TestHelper::new();

        t.run(SceneGenerator);

        let camera = t.entity("camera-1");

        assert_eq!(t.has::<Camera>(camera), true);
        assert_eq!(t.has::<Transform>(camera), true);
        assert_eq!(t.has::<FieldOfView>(camera), true);
        assert_eq!(t.has::<Image>(camera), true);
    }
}
