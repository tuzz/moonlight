//use super::*;
//use cgmath::Vector2;
//use std::f64::INFINITY;
//
//type Subject = ImageWriter;
//
//mod write {
//    use super::*;
//
//    #[test]
//    fn it_writes_the_image_to_a_file() {
//        let resolution = Vector2::new(20, 20);
//
//        let image = Image::new(resolution);
//        let name = Name::new("test-file");
//
//        Subject::write((&image, &name));
//    }
//}
//
//mod byte {
//    use super::*;
//
//    #[test]
//    fn it_normalizes_the_f64_to_the_0_to_255_range() {
//        assert_eq!(Subject::byte(0.2), 51);
//        assert_eq!(Subject::byte(0.5), 128);
//        assert_eq!(Subject::byte(0.7), 179);
//    }
//
//    #[test]
//    fn it_clamps_negative_values_to_zero() {
//        assert_eq!(Subject::byte(-0.1), 0);
//        assert_eq!(Subject::byte(-100.0), 0);
//        assert_eq!(Subject::byte(-INFINITY), 0);
//    }
//
//    #[test]
//    fn it_clamps_positive_values_to_255() {
//        println!("{}", INFINITY as i32);
//
//        assert_eq!(Subject::byte(1.1), 255);
//        assert_eq!(Subject::byte(100.0), 255);
//        assert_eq!(Subject::byte(INFINITY), 255);
//    }
//}
