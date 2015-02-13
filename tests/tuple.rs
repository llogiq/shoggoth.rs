#[cfg(test)]
mod test {
    extern crate shoggoth;

    use self::shoggoth::hlist::*;
    use self::shoggoth::nat;
    use self::shoggoth::syntax::product::{
        ProductOps,
    };
    use self::shoggoth::syntax::std::tuple::{
        TupleOps,
    };

    #[test]
    fn at() {
        assert_eq!((0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)).at::<nat::_0>(),
                    0u8);
    }

    #[test]
    fn to_hlist() {
        assert_eq!((0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)).to_hlist(),
                   Cons(0u8,
                   Cons(true,
                   Cons("foo",
                   Cons(Some(42u64),
                   Cons(Ok::<_, ()>(false),
                   Nil))))));
    }

    #[test]
    fn to_pair() {
        assert_eq!((0u8,  true, "foo", Some(42u64), Ok::<_, ()>(false)).to_pair(),
                   (0u8, (true, "foo", Some(42u64), Ok::<_, ()>(false))));
    }

    #[test]
    fn to_tuple() {
        assert_eq!((0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)).to_tuple(),
                   (0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)));
    }
}
