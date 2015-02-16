#[cfg(test)]
mod test {
    extern crate shoggoth;

    use self::shoggoth::list::*;
    use self::shoggoth::syntax::product::{
        ProductOps,
    };

    #[test]
    fn to_list() {
        assert_eq!((0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)).to_list(),
                   Cons(0u8,
                   Cons(true,
                   Cons("foo",
                   Cons(Some(42u64),
                   Cons(Ok::<_, ()>(false),
                   Nil))))));
    }

    #[test]
    fn to_tuple() {
        assert_eq!((0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)).to_tuple(),
                   (0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)));
    }
}
