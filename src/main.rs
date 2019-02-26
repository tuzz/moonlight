#[macro_use]
extern crate specs_derive;

#[cfg(test)] #[macro_use]
extern crate assert_approx_eq;

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

    for _ in 0..720 {
        RayTracer.run_now(&world.res);
        ImageWriter.run_now(&world.res);
        Physics.run_now(&world.res);
        FrameCounter.run_now(&world.res);
    }
}
