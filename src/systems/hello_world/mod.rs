use specs::prelude::*;
use crate::components::Name;

pub struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Name>;

    fn run(&mut self, name: Self::SystemData) {
        for name in name.join() {
            println!("Hello {}", name.s);
        }
    }
}
