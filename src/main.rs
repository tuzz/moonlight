#[macro_use]
extern crate specs_derive;

mod components;
mod systems;

use specs::prelude::*;
use components::*;
use systems::*;

fn main() {
    let mut world = World::new();
    register_components(&mut world);

    SceneGenerator.run_now(&world.res);
    world.maintain();

    RayTracer.run_now(&world.res);
    ImageWriter.run_now(&world.res);
}
