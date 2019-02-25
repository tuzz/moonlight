mod components;
mod systems;

use specs::{World, Builder, RunNow};
use components::Name;
use systems::HelloWorld;

fn main() {
    let mut world = World::new();

    world.register::<Name>();

    world.create_entity()
        .with(Name { s: "Chris".to_string() })
        .build();

    HelloWorld.run_now(&world.res);

    world.maintain();
}