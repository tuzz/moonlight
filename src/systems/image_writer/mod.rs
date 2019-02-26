use specs::prelude::*;
use std::{fs::File, io::BufWriter, iter::once};
use png::{Encoder, HasParameters};
use crate::components::{Image, Name, Frame};

pub struct ImageWriter;

impl<'a> System<'a> for ImageWriter {
    type SystemData = (
        ReadStorage<'a, Image>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Frame>,
    );

    fn run(&mut self, (image, name, frame): Self::SystemData) {
        (&image, &name, &frame).join().for_each(Self::write);
    }
}

impl ImageWriter {
    fn write((image, name, frame): (&Image, &Name, &Frame)) {
        let suffix = format!("-{:04}.png", frame.number);
        let filename = name.string.clone() + &suffix;

        let file = File::create(filename).expect("failed to create image file");
        let buffer = BufWriter::new(file);

        let mut encoder = Encoder::new(buffer, image.resolution.x, image.resolution.y);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);

        let mut writer = encoder.write_header().expect("failed to write png header");
        writer.write_image_data(&Self::bytes(image)).expect("failed to write png data");
    }

    fn bytes(image: &Image) -> Vec<u8> {
        image.pixels.iter().flat_map(|pixel| {

            once(Self::byte(pixel.x))
                .chain(once(Self::byte(pixel.y)))
                .chain(once(Self::byte(pixel.z)))

        }).collect()
    }

    fn byte(mut channel: f64) -> u8 {
        if channel > 1.0 { channel = 1.0 }
        if channel < 0.0 { channel = 0.0 }

        (channel * 255.0).round() as u8
    }
}

#[cfg(test)]
mod test;
