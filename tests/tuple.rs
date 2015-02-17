#[macro_use] extern crate shoggoth;

#[cfg(test)]
mod product {
    use super::shoggoth::syntax::product::{
        ProductOps,
    };

    #[test]
    fn to_list() {
        assert_eq!((0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)).to_list(),
              list![0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)]);
    }

    #[test]
    fn to_tuple() {
        let xs = (0u8, true, "foo", Some(42u64), Ok::<_, ()>(false));
        assert_eq!(xs.to_tuple(), xs);
    }
}
