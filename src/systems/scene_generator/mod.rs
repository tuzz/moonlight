use specs::prelude::*;
use crate::components::Name;

pub struct SceneGenerator;

impl<'a> System<'a> for SceneGenerator {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, lazy): Self::SystemData) {
        let chris = entities.create();

        lazy.insert(chris, Name { s: "Chris".to_string() });
    }
}
