use super::*;

type Subject = Image;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_flat_vector_of_pixels() {
        let resolution = Vector2::new(20, 10);
        let subject = Subject::new(resolution);

        assert_eq!(subject.pixels.len(), 200);
    }

    #[test]
    fn it_sets_the_pixel_color_to_black() {
        let resolution = Vector2::new(20, 10);
        let subject = Subject::new(resolution);

        assert_eq!(subject.pixels[0], Pixel::zero());
    }
}

mod get {
    use super::*;

    #[test]
    fn it_gets_the_pixel_at_a_coordinate() {
        let resolution = Vector2::new(20, 10);
        let subject = Subject::new(resolution);

        let coordinate = Vector2::new(3, 4);
        let pixel = subject.get(&coordinate);

        assert_eq!(pixel, &subject.pixels[4 * 20 + 3]);
    }
}

mod set {
    use super::*;

    #[test]
    fn it_sets_the_pixel_at_a_coordinate() {
        let resolution = Vector2::new(20, 10);
        let mut subject = Subject::new(resolution);

        let coordinate = Vector2::new(3, 4);
        let pixel = Pixel::new(0.1, 0.2, 0.3);

        subject.set(&coordinate, pixel);

        assert_eq!(subject.pixels[4 * 20 + 3], pixel);
    }
}
