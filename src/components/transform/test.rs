use super::*;

type Subject = Transform;

mod deref {
    use super::*;

    #[test]
    fn it_calls_through_to_the_matrix() {
        let matrix = Matrix4::from([[0.0; 4]; 4]);
        let subject = Subject::new(matrix);

        assert_eq!(subject[0][0], 0.0);
    }
}
