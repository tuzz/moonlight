use super::*;

type Subject = Transform;

mod new {
    use super::*;

    // For now, we always compute the inverse matrix for a transform, but we could
    // make this more sophisticated later, e.g. convert it lazily on first use and
    // cache it in the Transform. I think this is fine for now.

    #[test]
    fn it_stores_the_matrix_and_its_inverse() {
        let matrix = Matrix4::identity();
        let subject = Subject::new(matrix);

        assert_eq!(subject.matrix, matrix);
        assert_eq!(subject.inverse, matrix);
    }
}

mod deref {
    use super::*;

    #[test]
    fn it_calls_through_to_the_matrix() {
        let matrix = Matrix4::identity();
        let subject = Subject::new(matrix);

        assert_eq!(subject[0][0], 1.0);
        assert_eq!(subject[1][0], 0.0);
    }
}
