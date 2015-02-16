#[cfg(test)]
mod product {
    extern crate shoggoth;

    use self::shoggoth::list::*;
    use self::shoggoth::syntax::product::{
        ProductOps,
    };

    #[test]
    fn to_list() {
        let xs = (0u8, true, "foo", Some(42u64), Ok::<_, ()>(false));
        assert_eq!(xs.to_list(),
                   Cons(0u8,
                   Cons(true,
                   Cons("foo",
                   Cons(Some(42u64),
                   Cons(Ok::<_, ()>(false),
                   Nil))))));
    }

    #[test]
    fn to_tuple() {
        let xs = (0u8, true, "foo", Some(42u64), Ok::<_, ()>(false));
        assert_eq!(xs.to_tuple(), xs);
    }
}
