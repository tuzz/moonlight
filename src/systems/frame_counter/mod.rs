use specs::prelude::*;
use crate::components::Frame;

pub struct FrameCounter;

impl<'a> System<'a> for FrameCounter {
    type SystemData = WriteStorage<'a, Frame>;

    fn run(&mut self, mut frames: Self::SystemData) {
        println!("rendering next frame");

        for frame in (&mut frames).join() {
            frame.number += 1;
        }
    }
}
