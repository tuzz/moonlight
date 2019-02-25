#[macro_use]
extern crate specs_derive;

mod components;
mod systems;

use specs::prelude::*;
use components::Name;
use systems::{SceneGenerator, HelloWorld};

fn main() {
    let mut world = World::new();

    world.register::<Name>();

    SceneGenerator.run_now(&world.res);

    world.maintain();

    HelloWorld.run_now(&world.res);
}
