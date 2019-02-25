use super::*;

type Subject = Transform;

mod deref {
    use super::*;

    #[test]
    fn it_calls_through_to_the_matrix() {
        let matrix = Matrix4::identity();
        let subject = Subject::new(matrix);

        assert_eq!(subject[0], 1.0);
        assert_eq!(subject[1], 0.0);
    }
}
