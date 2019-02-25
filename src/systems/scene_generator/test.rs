use super::*;
use crate::systems::TestHelper;

type Subject = SceneGenerator;

mod run {
    use super::*;

    #[test]
    fn it_builds_an_entity_called_chris() {
        let mut t = TestHelper::new();

        t.run(SceneGenerator);

        assert_eq!(t.exists("Chris"), true);
    }
}
