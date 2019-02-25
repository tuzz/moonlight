use super::*;

type Subject = Transform;

mod deref {
    use super::*;

    #[test]
    fn it_calls_through_to_the_inner_field() {
        let inner = Transform3::identity();
        let subject = Subject::new(inner);

        assert_eq!(subject.matrix()[0], 1.0);
        assert_eq!(subject.matrix()[1], 0.0);
    }
}
